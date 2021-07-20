use std::{fs, path::Path, sync::Arc};

use anyhow::Result;
use ethers::prelude::*;

const BUILD_DIR: &'static str = env!("SOLC_BUILD_DIR");

pub fn get_factory<M: Middleware>(name: &str, client: &Arc<M>) -> Result<ContractFactory<M>> {
    let name = String::from(name);
    let build_dir = Path::new(BUILD_DIR);

    let abi_raw = fs::read_to_string(&build_dir.join(name.clone() + ".abi"))?;
    let abi = serde_json::from_str(&abi_raw)?;

    let bin_raw = fs::read_to_string(&build_dir.join(name + ".bin"))?;
    let bin: Bytes = hex::decode(&bin_raw)?.into();

    Ok(ContractFactory::new(abi, bin, client.clone()))
}
