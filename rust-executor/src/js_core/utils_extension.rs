use cid::Cid;
use deno_core::{error::AnyError, include_js_files, op, Extension};
use multibase::Base;
use multihash::{Code, MultihashDigest};
use tracing::{error, info, debug, warn};

use super::JS_CORE_HANDLE;

#[op]
fn hash(data: String) -> Result<String, AnyError> {
    // Compute the SHA-256 multihash
    let multihash = Code::Sha2_256.digest(data.as_bytes());

    // Create a CID with default settings (version 1, DAG-Protobuf)
    let cid = Cid::new_v1(0, multihash);

    // Encode the CID in base58btc (IPFS default)
    let encoded_cid = multibase::encode(Base::Base58Btc, cid.to_bytes());

    Ok(format!("Qm{}", encoded_cid))
}

#[op]
fn console_log(data: String) -> Result<String, AnyError> {
    info!("[JSCORE]: {:?}", data);

    Ok(String::from("temp"))
}

#[op]
fn console_debug(data: String) -> Result<String, AnyError> {
    debug!("[JSCORE]: {:?}", data);

    Ok(String::from("temp"))
}

#[op]
fn console_error(data: String) -> Result<String, AnyError> {
    error!("[JSCORE]: {:?}", data);

    Ok(String::from("temp"))
}

#[op]
fn console_warn(data: String) -> Result<String, AnyError> {
    warn!("[JSCORE]: {:?}", data);

    Ok(String::from("temp"))
}

#[op]
async fn load_module(path: String) -> Result<String, AnyError> {
    info!("Trying to load module: {}", path);

    let mut js_core_handle = JS_CORE_HANDLE.lock().await;

    if let Some(ref mut value) = *js_core_handle {
        // Call mutable functions on JsCoreHandle
        let _res = value.load_module(path).await;
    }

    Ok(String::from("temp"))
}

pub fn build() -> Extension {
    Extension::builder("utils")
        .js(include_js_files!(utils "utils_extension.js",))
        .ops(vec![
                hash::decl(),
                load_module::decl(),
                console_log::decl(),
                console_debug::decl(),
                console_error::decl(),
                console_warn::decl(),
            ])
        .force_op_registration()
        .build()
}
