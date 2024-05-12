mod wrappers;

use std::{path::PathBuf, str::FromStr};

use helios_client::ClientBuilder;
use helios_config::Network;
use wrappers::{EthAddress, EthU256Output, JsBlockTag, NapiEyreReport, SomeDB};

#[macro_use]
extern crate napi_derive;

const DEFAULT_CONSENSUS_RPC: &str = "https://www.lightclientdata.org";
const DEFAULT_EXECUTION_RPC: &str = "https://rpc.flashbots.net";

#[napi(js_name = "HeliosClient")]
pub struct JsHeliosClient {
  inner: helios_client::Client<wrappers::SomeDB>,
}

#[napi(constructor, js_name = "HeliosClientConfig")]
pub struct JsHeliosClientConfig {
  pub network: Option<String>,
  pub consensus_rpc: Option<String>,
  pub execution_rpc: Option<String>,
  pub data_dir: Option<String>,
  pub checkpoint: Option<String>,
}

#[napi]
impl JsHeliosClient {
  #[napi(factory)]
  pub fn with_config(config: &JsHeliosClientConfig) -> anyhow::Result<Self> {
    let _ = tracing_subscriber::fmt::fmt()
      .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
      .try_init();
    let mut client = ClientBuilder::new()
      .network(
        Network::from_str(config.network.as_deref().unwrap_or("mainnet"))
          .map_err(NapiEyreReport)?,
      )
      .consensus_rpc(
        config
          .consensus_rpc
          .as_deref()
          .unwrap_or(DEFAULT_CONSENSUS_RPC),
      )
      .execution_rpc(
        config
          .execution_rpc
          .as_deref()
          .unwrap_or(DEFAULT_EXECUTION_RPC),
      );

    if let Some(data_dir) = &config.data_dir {
      client = client.data_dir(PathBuf::from_str(data_dir)?);
    }

    if let Some(checkpoint) = &config.checkpoint {
      client = client.checkpoint(checkpoint);
    }

    let client = client.build::<SomeDB>().map_err(NapiEyreReport)?;
    Ok(Self { inner: client })
  }

  #[napi(
    ts_args_type = "address: string | Uint8Array, block_tag: BlockTag",
    ts_return_type = "Promise<bigint>"
  )]
  pub async fn get_balance(
    &self,
    address: EthAddress,
    block_tag: &JsBlockTag,
  ) -> napi::Result<EthU256Output> {
    let output = self
      .inner
      .get_balance(&address.0, block_tag.0)
      .await
      .map_err(NapiEyreReport)?;
    Ok(EthU256Output(output))
  }

  #[napi]
  pub async fn wait_synced(&self) {
    self.inner.wait_synced().await;
  }
}
