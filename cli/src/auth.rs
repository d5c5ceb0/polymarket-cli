use std::str::FromStr;

use anyhow::{Context, Result};
use polymarket_client_sdk::auth::LocalSigner;
use polymarket_client_sdk::auth::Normal;
use polymarket_client_sdk::auth::Signer as _;
use polymarket_client_sdk::auth::state::Authenticated;
use polymarket_client_sdk::{POLYGON, clob};

use crate::config;

#[allow(dead_code)]
pub async fn authenticated_clob_client(
    private_key: Option<&str>,
) -> Result<clob::Client<Authenticated<Normal>>> {
    let (key, _source) = config::resolve_key(private_key);
    let key = key.ok_or_else(|| anyhow::anyhow!("{}", config::NO_WALLET_MSG))?;

    let signer = LocalSigner::from_str(&key)
        .context("Invalid private key")?
        .with_chain_id(Some(POLYGON));

    let client = clob::Client::default();
    let authenticated = client
        .authentication_builder(&signer)
        .authenticate()
        .await
        .context("Failed to authenticate with Polymarket CLOB")?;

    Ok(authenticated)
}
