use dialoguer::Input;
use std::str::FromStr;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct Login {
    ///What is the name of the network
    network_name: String,
}

impl Login {
    pub async fn process(&self) -> crate::CliResult {
        let connection_config = match self.network_name.as_str() {
            "testnet" => crate::common::ConnectionConfig::Testnet,
            "mainnet" => crate::common::ConnectionConfig::Mainnet,
            "betanet" => crate::common::ConnectionConfig::Betanet,
            _ => todo!(),
        };
        login(connection_config).await
    }
}

async fn login(connection_config: crate::common::ConnectionConfig) -> crate::CliResult {
    let key_pair_properties: crate::common::KeyPairProperties =
        crate::common::generate_keypair().await?;
    let mut url: url::Url = connection_config.wallet_url().join("login/")?;
    url.query_pairs_mut()
        .append_pair("title", "NEAR CLI")
        .append_pair("public_key", &key_pair_properties.public_key_str);
    // Use `success_url` once capture mode is implemented
    //.append_pair("success_url", "http://127.0.0.1:8080");
    println!(
        "If your browser doesn't automatically open, please visit this URL:\n {}\n",
        &url.as_str()
    );
    // url.open();
    open::that(url.as_ref()).ok();

    let public_key: near_crypto::PublicKey =
        near_crypto::PublicKey::from_str(&key_pair_properties.public_key_str)?;

    let account_id = get_account_from_cli(public_key, connection_config.clone()).await?;
    // save_account(&account_id, key_pair_properties, self.connection_config).await?
    crate::common::save_access_key_to_keychain(
        Some(connection_config),
        key_pair_properties.clone(),
        &account_id.to_string(),
    )
    .await
    .map_err(|err| {
        color_eyre::Report::msg(format!("Failed to save a file with access key: {}", err))
    })?;
    Ok(())
}

async fn get_account_from_cli(
    public_key: near_crypto::PublicKey,
    network_connection_config: crate::common::ConnectionConfig,
) -> color_eyre::eyre::Result<near_primitives::types::AccountId> {
    let account_id = input_account_id()?;
    verify_account_id(account_id.clone(), public_key, network_connection_config)
        .await
        .map_err(|err| color_eyre::Report::msg(format!("Failed account ID: {:?}", err)))?;
    Ok(account_id)
}

fn input_account_id() -> color_eyre::eyre::Result<near_primitives::types::AccountId> {
    Ok(Input::new()
        .with_prompt("Enter account ID")
        .interact_text()?)
}

async fn verify_account_id(
    account_id: near_primitives::types::AccountId,
    public_key: near_crypto::PublicKey,
    network_connection_config: crate::common::ConnectionConfig,
) -> crate::CliResult {
    near_jsonrpc_client::JsonRpcClient::connect(network_connection_config.rpc_url())
        .call(near_jsonrpc_client::methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::Finality::Final.into(),
            request: near_primitives::views::QueryRequest::ViewAccessKey {
                account_id,
                public_key,
            },
        })
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!(
                "Failed to fetch query for view access key: {:?}",
                err
            ))
        })?;
    Ok(())
}