use anyhow::Result;
use futures::future::{BoxFuture, Future};

use ethers::{
    prelude::*,
    core::k256::ecdsa::SigningKey,
};

abigen!(
    ERC20MinterPauser,
    "./build/ERC20MinterPauser.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug, Clone)]
pub struct Context {
    pub token: ERC20MinterPauser<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub accts: Vec<LocalWallet>,
}

pub trait Runner {
    fn run(&self, arg: Context) -> BoxFuture<Result<()>>;
}

impl<T, F> Runner for T
where
    T: Fn(Context) -> F,
    F: Future<Output = Result<()>> + 'static + std::marker::Send,
{
    fn run(&self, args: Context) -> BoxFuture<Result<()>> {
        Box::pin(self(args))
    }
}
