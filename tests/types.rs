use anyhow::Result;
use async_trait::async_trait;
use futures::future::Future;
use std::sync::Arc;

use ethers::{core::k256::ecdsa::SigningKey, prelude::*};

abigen!(
    ERC20MinterPauser,
    "./build/ERC20MinterPauser.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub type Client = DevRpcMiddleware<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>;

#[derive(Debug, Clone)]
pub struct Ctx {
    pub client: Arc<Client>,
    pub accts: Vec<LocalWallet>,
    pub token: ERC20MinterPauser<Client>,
}

// Any async function from X -> Y. Can have side effects.
pub type AsyncMap<X, Y> = &'static (dyn AsyncAct<X, Y> + Send + Sync);
// An async function from X -> Maybe {} (side-effect only)
pub type Action<X> = AsyncMap<X, Result<()>>;

#[async_trait]
pub trait AsyncAct<X, Y> {
    async fn apply(&self, x: X) -> Y;
}
#[async_trait]
impl<F, X, Y, Fut> AsyncAct<X, Y> for F
where
    F: Fn(X) -> Fut + Sync,
    X: 'static + Send,
    Fut: Future<Output = Y> + Send,
{
    async fn apply(&self, x: X) -> Y {
        self(x).await
    }
}
