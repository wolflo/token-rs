use anyhow::Result;
use ethers::{
    prelude::*,
    utils::{Ganache, GanacheInstance, launch_ganache}
};
use std::{convert::TryFrom, sync::Arc};

mod utils;

abigen!(
    ERC20,
    "./build/ERC20.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    // launch and connect ganache
    let node: GanacheInstance = launch_ganache(Ganache::new()).await;
    let provider = Provider::<Http>::try_from(node.endpoint())?;
    let wallet: LocalWallet = node.keys()[0].clone().into();
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let factory = utils::get_factory("ERC20", &client)?;
    let deployed = factory.deploy( ( "Token".to_string(), "TOK".to_string() ) )?.send().await?;

    let token = ERC20::new(deployed.address(), client);

    let name = token.name().call().await?;
    let symbol = token.symbol().call().await?;
    assert_eq!(name, "Token");
    assert_eq!(symbol, "TOK");


    Ok(())
}
