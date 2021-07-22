use anyhow::{Result,Error};
use ethers::{
    prelude::*,
    core::k256::ecdsa::SigningKey,
    utils::{Ganache, GanacheInstance, launch_ganache}
};

use std::{convert::TryFrom, sync::Arc, panic::AssertUnwindSafe};
use core::pin::Pin;

use futures::{FutureExt, future::{Future, BoxFuture}};

mod utils;

abigen!(
    ERC20,
    "./build/ERC20.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug, Clone)]
struct Context
{
    token: ERC20<SignerMiddleware<Provider<Http>,Wallet<SigningKey>>>,
    accts: Vec<LocalWallet>
}

trait Runner {
    fn run<'a>(&self, arg: Context) -> BoxFuture<'a, Result<()>>;
}

impl<T, F> Runner for T
    where
        T: Fn(Context) -> F,
        F: Future<Output = Result<()>> + 'static + std::marker::Send
{
    fn run<'a>(&self, args: Context) -> BoxFuture<'a, Result<()>> {
        Box::pin(self(args))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let node: &GanacheInstance = &launch_ganache(Ganache::new()).await;
    let ctx = setup(&node, 3).await?;

    let tests: Vec< Box< dyn Runner > > = vec![
        Box::new(test_name),
        Box::new(test_symbol),
        Box::new(test_transfer),
    ];


    for (i, test) in tests.iter().enumerate() {
        let res = AssertUnwindSafe(
            test.run(ctx.clone()) // using &ctx here results in https://github.com/rust-lang/rust/issues/64650
        ).catch_unwind().await;
        println!(
            "test {} {}",
            i,
            match res { Ok(_) => "passed.", _ => "failed!" }
        );
    }

    Ok(())
}


async fn setup(node: &GanacheInstance, n_accts: usize) -> Result<Context> {
    let provider = Provider::<Http>::try_from(node.endpoint())?;
    let accts: Vec<LocalWallet> =
        node.keys()[..n_accts].iter()
        .map(|x| x.clone().into()).collect();

    let client = Arc::new(SignerMiddleware::new(provider, accts[0].clone()));

    let factory = utils::make_factory("ERC20", &client)?;
    let deployed = factory.deploy( ( "Token".to_string(), "TOK".to_string() ) )?.send().await?;

    let token = ERC20::new(deployed.address(), client);

    Ok( Context { token, accts })
}

async fn test_name(ctx: Context) -> Result<()> {
    let name = &ctx.token.name().call().await?;
    assert_eq!(name, "Token");
    Ok(())
}

async fn test_symbol(ctx: Context) -> Result<()> {
    let symbol = &ctx.token.symbol().call().await?;
    assert_eq!(symbol, "TOK");
    Ok(())
}

async fn test_transfer(ctx: Context) -> Result<()> {
    panic!("no tokens minted!");
}
