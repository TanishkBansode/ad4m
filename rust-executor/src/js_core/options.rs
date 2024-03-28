use deno_runtime::runtime;
use deno_runtime::worker::WorkerOptions;
use std::{collections::HashMap, rc::Rc};
use url::Url;

use super::{
    pubsub_extension, string_module_loader::StringModuleLoader, utils_extension,
    wallet_extension, signature_extension, agent_extension, languages_extension,
};
use crate::holochain_service::holochain_service_extension;
use crate::prolog_service::prolog_service_extension;
use crate::runtime_service::runtime_service_extension;

pub fn main_module_url() -> Url {
    Url::parse("https://ad4m.runtime/main").unwrap()
}
#[cfg(not(target_os = "windows"))]
pub fn module_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(
        "https://ad4m.runtime/main".to_string(),
        include_str!("main.js").to_string(),
    );

    map.insert(
        "https://ad4m.runtime/executor".to_string(),
        include_str!("../../executor/lib/bundle.js").to_string(),
    );
    map
}

#[cfg(target_os = "windows")]
pub fn module_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(
        "https://ad4m.runtime/main".to_string(),
        include_str!("main.js").to_string(),
    );

    map.insert(
        "https://ad4m.runtime/executor".to_string(),
        include_str!("../../../executor/lib/bundle.js").to_string(),
    );
    map
}

pub fn main_worker_options() -> WorkerOptions {
    let mut loader = StringModuleLoader::new();
    for (specifier, code) in module_map() {
        loader.add_module(specifier.as_str(), code.as_str());
    }

    let wallet_ext = wallet_extension::build();
    let utils_ext = utils_extension::build();
    let sub_ext = pubsub_extension::build();
    let holochain_ext = holochain_service_extension::build();
    let prolog_ext = prolog_service_extension::build();
    let signature_ext = signature_extension::build();
    let agent_ext = agent_extension::build();
    let runtime_ext = runtime_service_extension::build();
    let languages_ext = languages_extension::build();

    WorkerOptions {
        extensions: vec![
            wallet_ext,
            utils_ext,
            sub_ext,
            holochain_ext,
            prolog_ext,
            signature_ext,
            agent_ext,
            runtime_ext
            languages_ext,
        ],
        module_loader: Rc::new(loader),
        ..Default::default()
    }
}
