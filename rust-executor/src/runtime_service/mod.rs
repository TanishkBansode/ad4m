use std::{fs::File, sync::Mutex};
use std::io::{Read};
pub(crate) mod runtime_service_extension;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapSeed {
    #[serde(rename = "trustedAgents")]
    pub trusted_agents: Vec<String>,
    #[serde(rename = "knownLinkLanguages")]
    pub known_link_languages: Vec<String>,
    #[serde(rename = "directMessageLanguage")]
    pub direct_message_language: String,
    #[serde(rename = "agentLanguage")]
    pub agent_language: String,
    #[serde(rename = "perspectiveLanguage")]
    pub perspective_language: String,
    #[serde(rename = "neighbourhoodLanguage")]
    pub neighbourhood_language: String,
    #[serde(rename = "languageLanguageBundle")]
    pub language_language_bundle: String,
}

use serde::{Deserialize, Serialize};

use crate::graphql::graphql_types::NotificationInput;
use crate::pubsub::{get_global_pubsub, RUNTIME_INSTALL_NOTIFICATION_REQUESTED_TOPIC};
use crate::{agent::did, db::Ad4mDb, graphql::graphql_types::SentMessage};

lazy_static! {
    static ref RUNTIME_INSTANCE: Arc<Mutex<Option<RuntimeService>>> = Arc::new(Mutex::new(None));
}

pub struct RuntimeService {
    seed: BootstrapSeed,
}

impl RuntimeService {
    pub fn init_global_instance(mainnet_seed_file_path: String) {
        let mut instance = RUNTIME_INSTANCE.lock().unwrap();
        *instance = Some(RuntimeService::new(mainnet_seed_file_path));
    }

    fn new(mainnet_seed_file_path: String) -> RuntimeService {
        let mut file = File::open(&mainnet_seed_file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let seed: BootstrapSeed = serde_json::from_str(&contents).unwrap();

        RuntimeService {
            seed
        }
    }

    pub fn global_instance() -> Arc<Mutex<Option<RuntimeService>>> {
        RUNTIME_INSTANCE.clone()
    }

    pub fn with_global_instance<F, R>(func: F) -> R
    where
        F: FnOnce(&RuntimeService) -> R,
    {
        let global_instance_arc = RuntimeService::global_instance();
        let lock_result = global_instance_arc.lock();
        let runtime_lock = lock_result.expect("Couldn't get lock on Ad4mDb");
        let runtime_ref = runtime_lock.as_ref().expect("Ad4mDb not initialized");
        func(runtime_ref)
    }

    pub fn get_trusted_agents(&self) -> Vec<String> {
        let mut trusted_agents: Vec<String> = self.seed.trusted_agents.clone();
        let mut stored_agents = Ad4mDb::with_global_instance(|db| {
            db.get_all_trusted_agents()
        }).map_err(|e| e.to_string()).unwrap_or(vec![]);
        trusted_agents.push(did());
        trusted_agents.append(&mut stored_agents);
        trusted_agents.sort();
        trusted_agents.dedup();
        trusted_agents
    }

    pub fn add_trusted_agent(&self, new_agents: Vec<String>) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.add_trusted_agents(new_agents)
        }).map_err(|e| e.to_string());
    }

    pub fn remove_trusted_agent(&self, agents_to_remove: Vec<String>) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.remove_trusted_agents(agents_to_remove)
        }).map_err(|e| e.to_string());
    }

    pub fn get_know_link_languages(&self) -> Vec<String> {
        let mut languages: Vec<String> = self.seed.known_link_languages.clone();
        let mut stored_languages = Ad4mDb::with_global_instance(|db| {
            db.get_all_known_link_languages()
        }).map_err(|e| e.to_string()).unwrap_or(vec![]);
        languages.append(&mut stored_languages);
        languages.sort();
        languages.dedup();
        languages
    }

    pub fn add_know_link_language(&self, language: Vec<String>) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.add_known_link_languages(language)
        }).map_err(|e| e.to_string());
    }

    pub fn remove_know_link_language(&self, language_to_remove: Vec<String>) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.remove_known_link_languages(language_to_remove)
        }).map_err(|e| e.to_string());
    }

    pub fn get_friends(&self) -> Vec<String> {
        let friends = Ad4mDb::with_global_instance(|db| {
            db.get_all_friends()
        }).map_err(|e| e.to_string()).unwrap_or(vec![]);

        friends
    }

    pub fn add_friend(&self, friends: Vec<String>) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.add_friends(friends)
        }).map_err(|e| e.to_string());
    }

    pub fn remove_friend(&self, friend_to_remove: Vec<String>) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.remove_friends(friend_to_remove)
        }).map_err(|e| e.to_string());
    }

    pub fn get_outbox(&self) -> Vec<SentMessage> {
        let outbox = Ad4mDb::with_global_instance(|db| {
            db.get_all_from_outbox()
        }).map_err(|e| e.to_string()).unwrap_or(vec![]);

        outbox
    }

    pub fn add_message_to_outbox(&self, message: SentMessage) {
        let _ = Ad4mDb::with_global_instance(|db| {
            db.add_to_outbox(&message.message, message.recipient)
        }).map_err(|e| e.to_string());
    }


    pub async fn request_install_notification(notification_input: NotificationInput) -> Result<(), String>{
        let notification_id = Ad4mDb::with_global_instance(|db| {
            db.add_notification(notification_input)
        }).map_err(|e| e.to_string())?;

        let notification =Ad4mDb::with_global_instance(|db| {
             db.get_notification(notification_id)
        }).map_err(|e| e.to_string())?.ok_or("Notification with given id not found")?;

        get_global_pubsub()
            .await
            .publish(
                &RUNTIME_INSTALL_NOTIFICATION_REQUESTED_TOPIC,
                &serde_json::to_string(&notification).unwrap(),
            )
            .await;

        Ok(())
    }

    

}
