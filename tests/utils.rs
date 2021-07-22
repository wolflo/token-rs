use anyhow::Result;
use std::{fs, sync::Arc, path::Path, convert::TryFrom};

use ethers::{
    prelude::*,
    utils::GanacheInstance,
};

use crate::types::*;

const BUILD_DIR: &'static str = env!("SOLC_BUILD_DIR");

pub async fn setup(node: &GanacheInstance, n_accts: usize) -> Result<Context> {
    let provider = Provider::<Http>::try_from(node.endpoint())?;
    let accts: Vec<LocalWallet> = node.keys()[..n_accts]
        .iter()
        .map(|x| x.clone().into())
        .collect();

    let client = Arc::new(SignerMiddleware::new(provider, accts[0].clone()));

    let factory = make_factory("ERC20MinterPauser", &client)?;
    let deployed = factory
        .deploy(("Token".to_string(), "TOK".to_string()))?
        .send()
        .await?;

    let token = ERC20MinterPauser::new(deployed.address(), client);

    Ok(Context { token, accts })
}

pub fn make_factory<M: Middleware>(name: &str, client: &Arc<M>) -> Result<ContractFactory<M>> {
    let name = String::from(name);
    let build_dir = Path::new(BUILD_DIR);

    let abi_raw = fs::read_to_string(&build_dir.join(name.clone() + ".abi"))?;
    let abi = serde_json::from_str(&abi_raw)?;

    let bin_raw = fs::read_to_string(&build_dir.join(name + ".bin"))?;
    let bin: Bytes = hex::decode(&bin_raw)?.into();

    Ok(ContractFactory::new(abi, bin, client.clone()))
}
