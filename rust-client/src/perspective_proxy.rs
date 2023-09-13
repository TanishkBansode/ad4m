use std::collections::BTreeMap;

use crate::{
    literal::{Literal, LiteralValue},
    perspectives::{
        add_link::AddLinkPerspectiveAddLink, query_links::QueryLinksPerspectiveQueryLinks,
        PerspectivesClient,
    },
    subject_proxy::SubjectProxy,
    types::LinkExpression,
};
use anyhow::{anyhow, Result};
use chrono::naive::NaiveDateTime;
use regex::Regex;
use serde_json::Value;
type DateTime = NaiveDateTime;

pub struct PerspectiveProxy {
    client: PerspectivesClient,
    perspective_uuid: String,
}

impl PerspectiveProxy {
    pub fn new(client: PerspectivesClient, perspective_uuid: String) -> Self {
        Self {
            client,
            perspective_uuid,
        }
    }

    pub async fn add_link(
        &self,
        source: String,
        target: String,
        predicate: Option<String>,
        status: Option<String>
    ) -> Result<AddLinkPerspectiveAddLink> {
        self.client
            .add_link(self.perspective_uuid.clone(), source, target, predicate, status)
            .await
    }

    pub async fn get(
        &self,
        source: Option<String>,
        target: Option<String>,
        predicate: Option<String>,
        from_date: Option<DateTime>,
        until_date: Option<DateTime>,
        limit: Option<f64>,
    ) -> Result<Vec<QueryLinksPerspectiveQueryLinks>> {
        self.client
            .query_links(
                self.perspective_uuid.clone(),
                source,
                target,
                predicate,
                from_date,
                until_date,
                limit,
            )
            .await
    }

    pub async fn infer(&self, prolog_query: String) -> Result<Value> {
        self.client
            .infer(self.perspective_uuid.clone(), prolog_query)
            .await
    }

    pub async fn set_dna(&self, dna: String) -> Result<()> {
        let literal = Literal::from_string(dna);
        self.set_single_target(
            "ad4m://self".into(),
            "ad4m://has_zome".into(),
            literal.to_url()?,
        )
        .await?;
        Ok(())
    }

    pub async fn get_dna(&self) -> Result<Vec<String>> {
        self.get(
            Some("ad4m://self".into()),
            None,
            Some("ad4m://has_zome".into()),
            None,
            None,
            None,
        )
        .await?
        .into_iter()
        .map(|link| {
            let literal = Literal::from_url(link.data.target)?;
            match literal.get() {
                Ok(LiteralValue::String(string)) => Ok(string),
                _ => Err(anyhow::anyhow!("Not a string literal")),
            }
        })
        .collect()
    }

    pub async fn get_single_target(&self, source: String, predicate: String) -> Result<String> {
        let links = self
            .client
            .query_links(
                self.perspective_uuid.clone(),
                Some(source),
                None,
                Some(predicate),
                None,
                None,
                None,
            )
            .await?;
        if links.is_empty() {
            return Err(anyhow::anyhow!("No links found"));
        }
        if links.len() > 1 {
            return Err(anyhow::anyhow!("Multiple links found"));
        }
        Ok(links[0].data.target.clone())
    }

    pub async fn set_single_target(
        &self,
        source: String,
        predicate: String,
        target: String,
    ) -> Result<()> {
        let links: Vec<LinkExpression> = self
            .client
            .query_links(
                self.perspective_uuid.clone(),
                Some(source.clone()),
                None,
                Some(predicate.clone()),
                None,
                None,
                None,
            )
            .await?
            .into_iter()
            .map(LinkExpression::from)
            .collect();

        for link in links {
            self.client
                .remove_link(self.perspective_uuid.clone(), link)
                .await?;
        }

        self.client
            .add_link(
                self.perspective_uuid.clone(),
                source,
                target,
                Some(predicate),
                Some("shared".to_string())
            )
            .await?;
        Ok(())
    }

    pub async fn subject_classes(&self) -> Result<Vec<String>> {
        let mut classes = Vec::new();
        if let Value::Array(classes_results) = self.infer("subject_class(X, _)".into()).await? {
            for class in classes_results {
                if let Some(Value::String(class_name)) = class.get("X") {
                    classes.push(class_name.clone());
                }
            }
        }
        Ok(classes)
    }

    pub async fn subject_class_properties(&self, class: &String) -> Result<Vec<String>> {
        let mut propertys = Vec::new();
        if let Value::Array(propertys_results) = self
            .infer(format!("subject_class(\"{}\", C), property(C, X)", class))
            .await?
        {
            for property in propertys_results {
                if let Some(Value::String(property_name)) = property.get("X") {
                    propertys.push(property_name.clone());
                }
            }
        }
        Ok(propertys)
    }

    pub async fn subject_class_collections(&self, class: &String) -> Result<Vec<String>> {
        let mut collections = Vec::new();
        if let Value::Array(collection_results) = self
            .infer(format!("subject_class(\"{}\", C), collection(C, X)", class))
            .await?
        {
            for colletion in collection_results {
                if let Some(Value::String(collection_name)) = colletion.get("X") {
                    collections.push(collection_name.clone());
                }
            }
        }
        Ok(collections)
    }

    pub async fn create_subject(&self, class: &String, base: &String) -> Result<()> {
        if let Ok(Value::Array(results)) = self
            .infer(format!(
                "subject_class(\"{}\", C), constructor(C, Action)",
                class
            ))
            .await
        {
            //println!("{:?}", results);
            if let Some(Value::Object(action)) = results.first() {
                //println!("{:?}", action);
                if let Value::String(action_string) = action
                    .get("Action")
                    .ok_or(anyhow::anyhow!("Unbound variable Action is not set"))?
                {
                    //println!("{}", action_string);
                    self.execute_action(action_string, base, None).await?;
                    return Ok(());
                }
            }
        }
        Err(anyhow::anyhow!("No constructor found for class: {}", class))
    }

    pub async fn is_subject_instance(&self, class: &String, base: &String) -> Result<bool> {
        match self
            .infer(format!(
                r#"subject_class("{}", C), instance(C, "{}")"#,
                class, base
            ))
            .await?
        {
            Value::Array(results) => {
                if results.is_empty() {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Value::Bool(b) => Ok(b),
            _ => Ok(false),
        }
    }

    pub async fn get_subject(&self, class: &String, base: &String) -> Result<SubjectProxy> {
        if self.is_subject_instance(class, base).await? {
            Ok(SubjectProxy::new(self.clone(), class.clone(), base.clone()))
        } else {
            Err(anyhow!(
                "Expression {} is not a subject instance of class: {}",
                base,
                class
            ))
        }
    }

    pub async fn get_subject_classes(&self, base: &String) -> Result<Vec<String>> {
        let mut classes = Vec::new();
        if let Value::Array(classes_results) = self
            .infer(format!(
                r#"instance(C, "{}"), subject_class(Classname, C)"#,
                base
            ))
            .await?
        {
            for class in classes_results {
                if let Some(Value::String(class_name)) = class.get("Classname") {
                    classes.push(class_name.clone());
                }
            }
        }
        Ok(classes)
    }

    pub async fn execute_action(
        &self,
        action: &String,
        base: &String,
        params: Option<BTreeMap<&str, &String>>,
    ) -> Result<()> {
        let commands = parse_action(action)?;
        for command in commands {
            let mut command = command.replace("this", base);
            if let Some(ref params) = params {
                for (key, value) in params.iter() {
                    command = command.replace(key, value);
                }
            }
            match command.action.as_str() {
                "addLink" => {
                    //println!("addLink: {:?}", command);
                    self.add_link(command.source, command.target, command.predicate, command.status)
                        .await?;
                }
                "removeLink" => {
                    unimplemented!();
                    //let links = self.get(Some(source), Some(target), predicate, None, None, None).await?;
                    //elf.remove_link(source, target.into(), predicate.into()).await?;
                }
                "setSingleTarget" => {
                    self.set_single_target(
                        command.source,
                        command
                            .predicate
                            .ok_or(anyhow::anyhow!("No predicate in set_single_target action"))?,
                        command.target,
                    )
                    .await?;
                }
                _ => {
                    return Err(anyhow::anyhow!("Unknown action: {}", command.action));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Command {
    pub action: String,
    pub source: String,
    pub predicate: Option<String>,
    pub target: String,
    pub status: Option<String>
}

impl Command {
    pub fn replace(&self, pattern: &str, replacement: &str) -> Command {
        Command {
            action: self.action.replace(pattern, replacement),
            source: self.source.replace(pattern, replacement),
            predicate: self
                .predicate
                .as_ref()
                .map(|f| f.replace(pattern, replacement)),
            target: self.target.replace(pattern, replacement),
            status: self
                .status
                .as_ref()
                .map(|f| f.replace(pattern, replacement)),
        }
    }
}

fn parse_action(action: &String) -> Result<Vec<Command>> {
    let action_regex = Regex::new(r"\[(?P<command>\{.*})*\]")?;

    // This parses strings like:
    // {action: "<action>", source: "<source>", predicate: "<predicate>", target: "<target>", status: "<status>"}
    let command_regex = Regex::new(
        r#"\{(action:\s*"(?P<action>[\S--,]+)",?\s*)|(source:\s*"(?P<source>[\S--,]+)",?\s*)|(predicate:\s*"(?P<predicate>[\S--,]+)",?\s*)|(target:\s*"(?P<target>[\S--,]+)",?\s*)|(status:\s*"(?P<status>[\S--,]+)",?\s*)\}"#,
    )?;

    let mut commands = Vec::new();
    for capture in action_regex.captures_iter(action) {
        let mut action = None;
        let mut source = None;
        let mut predicate = None;
        let mut target = None;
        let mut status = None;
        command_regex
            .captures_iter(capture.name("command").unwrap().as_str())
            .for_each(|capture| {
                action = action.or(capture.name("action").map(|e| e.as_str()));
                source = source.or(capture.name("source").map(|e| e.as_str()));
                predicate = predicate.or(capture.name("predicate").map(|e| e.as_str()));
                target = target.or(capture.name("target").map(|e| e.as_str()));
                status = status.or(capture.name("status").map(|e| e.as_str()));
            });

        commands.push(Command {
            action: action.ok_or(anyhow!("Comman without action"))?.into(),
            source: source.ok_or(anyhow!("Comman without source"))?.into(),
            predicate: predicate.map(|e| e.into()),
            target: target.ok_or(anyhow!("Comman without target"))?.into(),
            status: status.map(|e| e.into()),
        });
        //println!("{:?}", commands);
    }

    Ok(commands)
}
