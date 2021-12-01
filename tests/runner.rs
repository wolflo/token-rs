use anyhow::Result;
use async_trait::async_trait;
use futures::future::Future;

type Action<X> = &'static (dyn AsyncAct<X, Result<()>> + Sync + Send);
#[async_trait]
trait AsyncAct<X, Y> {
    async fn apply(&self, domain: X) -> Y;
}
#[async_trait]
impl<F, X, Y, Fut> AsyncAct<X, Y> for F
where
    F: Fn(X) -> Fut + Sync,
    X: 'static + Send,
    Fut: Future<Output = Y> + Send
{
    async fn apply(&self, x: X) -> Y {
        self(x).await
    }
}


use ethers::{
    prelude::*,
    utils::{GanacheInstance, Ganache, launch_ganache},
    core::k256::ecdsa::SigningKey,
};
use std::{fs, sync::Arc, path::Path, convert::TryFrom, time::Duration};
mod erc20_mintable_pausable;
use erc20_mintable_pausable::*;
abigen!(
    ERC20MinterPauser,
    "./build/ERC20MinterPauser.abi",
    event_derives(serde::Deserialize, serde::Serialize)
);
type Signer = SignerMiddleware<Provider<Http>,Wallet<SigningKey>>;
#[derive(Debug, Clone)]
pub struct Context {
    pub token: ERC20MinterPauser<Signer>,
    pub accts: Vec<LocalWallet>,
}
const BUILD_DIR: &'static str = env!("SOLC_BUILD_DIR");
pub async fn setup(node: &GanacheInstance, n_accts: usize) -> Result<Context> {
    let provider = Provider::<Http>::try_from(node.endpoint())?.interval(Duration::from_millis(1));
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

inventory::collect!(Action<Context>);

async fn test_foo(ctx: Context) -> Result<()> { println!("Testing foo."); Ok(()) }
async fn test_bar(ctx: Context) -> Result<()> { println!("Testing bar."); Ok(()) }

inventory::submit!(&test_foo as Action<Context>);
inventory::submit!(&test_bar as Action<Context>);
inventory::submit!(&test_name as Action<Context>);
inventory::submit!(&test_symbol as Action<Context>);
inventory::submit!(&test_mint as Action<Context>);
inventory::submit!(&test_transfer as Action<Context>);


#[tokio::main]
async fn main() -> Result<()> {
    let node: &GanacheInstance = &launch_ganache(Ganache::new()).await;
    let ctx = setup(&node, 3).await?;
    for act in inventory::iter::<Action<Context>> {
        act.apply(ctx.clone()).await?;
    }
    Ok(())
}
