extern crate async_tungstenite;
extern crate clap;
extern crate anyhow;
extern crate graphql_client;
extern crate reqwest;
extern crate tokio;
extern crate rustyline;
extern crate dirs;
extern crate chrono;

mod agent;
mod formatting;
mod perspectives;
mod startup;
mod types;
mod util;

use clap::{Args, Parser, Subcommand};
use anyhow::{Context, Result, bail};
use startup::executor_data_path;
use util::{maybe_parse_datetime, readline_masked};
use serde_json::Value;
use formatting::{print_prolog_result, print_link, print_agent};

/// AD4M command line interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClapApp {
   /// Name of the person to greet
   #[command(subcommand)]
   domain: Domain,
}

#[derive(Debug, Subcommand)]
enum Domain {
    /// Functions related to local agent / user
    Agent {
        #[command(subcommand)]
        command: AgentFunctions,
    },

    /// See, create, and manage Languages
    Languages{
        #[command(subcommand)]
        command: Option<LanguageFunctions>,
    },

    /// Add, remove and access Perspectives / add and remove links
    Perspectives{
        #[command(subcommand)]
        command: Option<PerspectiveFunctions>,
    },
    Neighbourhoods{
        #[command(subcommand)]
        command: AgentFunctions,
    },
    Runtime{
        #[command(subcommand)]
        command: AgentFunctions,
    },
    Log
}

#[derive(Debug, Subcommand)]
enum AgentFunctions {
    Me,
    Status,
    Lock,
    Unlock,
}

#[derive(Debug, Subcommand)]
enum LanguageFunctions {
    ByAddress,
    ByFilter,
    All,
    WriteSettings,
    ApplyTemplateAndPublish,
    Publish,
    Meta,
    Source,
    Remove
}

#[derive(Debug, Subcommand)]
enum PerspectiveFunctions {
    /// Add a perspective with given name
    Add { name: String },

    /// Remove perspective with given uuid
    Remove { id: String },

    /// Add link to perspective with given uuid
    AddLink { id: String, source: String, target: String, predicate: Option<String>},

    /// Query links from perspective with given uuid
    QueryLinks(QueryLinksArgs),

    /// Run Prolog / SDNA query on perspective with given uuid
    Infer{ id: String, query: String },

    Watch { id: String }
}

#[derive(Args, Debug)]
struct QueryLinksArgs {
    /// Perspective ID
    id: String,

    /// Filter by source
    source: Option<String>,

    /// Filter by target
    target: Option<String>,

    /// Filter by predicate
    predicate: Option<String>,

    /// Get only links after this date (fromat: %Y-%m-%dT%H:%M:%S%.fZ)
    #[arg(short, long)]
    from_date: Option<String>,

    /// Get only links before this date (fromat: %Y-%m-%dT%H:%M:%S%.fZ)
    #[arg(short, long)]
    until_date: Option<String>,

    /// Get only the first n links
    #[arg(short, long)]
    limit: Option<f64>, 
}

#[derive(Debug, Subcommand)]
enum NeighbourhoodFunctions {
    Create { perspective_id: String, link_language: String },
    Join { url: String },
}

#[derive(Debug, Subcommand)]
enum RuntimeFunctions {
    Info,
    Quit,
    AddTrustedAgents { agents: Vec<String> },
    DeleteTrustedAgents { agents: Vec<String> },
    TrustedAgents,
    AddLinkLanguageTemplates { addresses: Vec<String> },
    RemoveLinkLanguageTemplates { addresses: Vec<String> },
    LinkLanguageTemplates,
    AddFriends { agents: Vec<String> },
    RemoveFriends { agents: Vec<String> },
    Friends,
    HcAgentInfos,
    HcAddAgentInfos { infos: Vec<String> },
    VerifySignature { did: String, did_signing_key: String, data: String, signed_data: String },
    SetStatus { status: String },
    FriendStatus { agent: String },
    FriendSendMessage { agent: String, message: String },
    MessageInbox { filter: Option<String> },
    MessageOutbox { filter: Option<String> },
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = ClapApp::parse();

    let cap_token = if let Domain::Log = args.domain {
        "".to_string()
    } else {
        startup::get_cap_token().await?
    };

    match args.domain {
        Domain::Agent{command} => {
            match command {
                AgentFunctions::Me => {
                    let agent = agent::run_me(cap_token).await?;
                    print_agent(agent.into());
                },
                AgentFunctions::Status => {
                    let status = agent::run_status(cap_token).await?;
                    println!("\x1b[36mDID: \x1b[97m{}", status.did.unwrap_or("<undefined>".to_string()));
                    println!("\x1b[36mis_initiliazed: \x1b[97m{}", status.is_initialized);
                    println!("\x1b[36mis_unlocked: \x1b[97m{}", status.is_unlocked);
                    println!("\x1b[36mDID Document:\n\x1b[97m{}", status.did_document.unwrap_or("<undefined>".to_string()));
                },
                AgentFunctions::Lock => {
                    let result = agent::run_lock(cap_token, readline_masked("Passphrase: ")?).await?;
                    if let Some(error) = result.error {
                        bail!(error);
                    } else {
                        println!("Agent locked");
                    }
                },
                AgentFunctions::Unlock => {
                    //agent::unlock(&cap_token).await?;
                },
            }
        },
        Domain::Languages{command: _} => {},
        Domain::Perspectives{command} => {
            if command.is_none() {
                let all_perspectives = perspectives::run_all(cap_token).await?;
                println!("{:#?}", all_perspectives);
                return Ok(());
            }

            match command.unwrap() {
                PerspectiveFunctions::Add { name } => {
                    let new_perspective = perspectives::run_add(cap_token, name).await?;
                    println!("{:#?}", new_perspective);
                },
                PerspectiveFunctions::Remove { id } => {
                    perspectives::run_remove(cap_token, id).await?;
                },
                PerspectiveFunctions::AddLink { id, source, target, predicate } => {
                    perspectives::run_add_link(cap_token, id, source, target, predicate).await?;
                },
                PerspectiveFunctions::QueryLinks(args) => {
                    let from_date = maybe_parse_datetime(args.from_date)?;
                    let until_date = maybe_parse_datetime(args.until_date)?;
                    let result = perspectives::run_query_links(cap_token, args.id, args.source, args.target, args.predicate, from_date, until_date, args.limit).await?;
                    for link in result {
                        print_link(link.into());
                    }
                },
                PerspectiveFunctions::Infer { id, query } => {
                    match perspectives::run_infer(cap_token, id, query).await? {
                        Value::Bool(true) => println!("true ✅"),
                        Value::Bool(false) => println!("false ❌"),
                        Value::String(string) => println!("{}", string),
                        Value::Array(array) => {
                            println!("\x1b[90m{} results:", array.len());
                            let mut i = 1;
                            for item in array {
                                println!("\x1b[90m{}:", i);
                                print_prolog_result(item)?;
                                println!("====================");
                                i += 1;
                            }
                        },
                        _ => bail!("Unexpected result value in response of run_infer()"),
                    }
                },
                PerspectiveFunctions::Watch { id } => {
                    perspectives::run_watch(cap_token, id).await?;
                }
            }
        },
        Domain::Neighbourhoods{command: _} => {},
        Domain::Runtime{command: _} => {},
        Domain::Log => {
            let file = executor_data_path().join("ad4min.log");
            let log = std::fs::read_to_string(file.clone())
                .with_context(||format!("Could not read log file `{}`!\nIs AD4M executor running?", file.display()))?;
            println!("{}", log);   
        }
    }

    Ok(())
}
