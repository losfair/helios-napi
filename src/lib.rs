mod wrappers;

use std::{path::PathBuf, str::FromStr};

use ethers::types::U256;
use helios_client::ClientBuilder;
use helios_config::Network;
use napi::bindgen_prelude::Buffer;
use wrappers::{
  EthAddress, EthU256Output, JsBlockTag, JsCallOpts, NapiAnyhowError, NapiEyreReport, SomeDB,
};

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

  #[napi]
  pub async fn wait_synced(&self) {
    self.inner.wait_synced().await;
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

  #[napi(ts_args_type = "address: string | Uint8Array, block_tag: BlockTag")]
  pub async fn get_code(
    &self,
    address: EthAddress,
    block_tag: &JsBlockTag,
  ) -> napi::Result<Buffer> {
    let output = self
      .inner
      .get_code(&address.0, block_tag.0)
      .await
      .map_err(NapiEyreReport)?;
    Ok(Buffer::from(output))
  }

  #[napi(
    ts_args_type = "address: string | Uint8Array, block_tag: BlockTag",
    ts_return_type = "Promise<bigint>"
  )]
  pub async fn get_nonce(
    &self,
    address: EthAddress,
    block_tag: &JsBlockTag,
  ) -> napi::Result<EthU256Output> {
    let output = self
      .inner
      .get_nonce(&address.0, block_tag.0)
      .await
      .map_err(NapiEyreReport)?;
    Ok(EthU256Output(U256::from(output)))
  }

  #[napi]
  pub async fn call(&self, call_opts: &JsCallOpts, block_tag: &JsBlockTag) -> napi::Result<Buffer> {
    let call_opts = (*call_opts.0.lock().unwrap()).clone();
    let output = self
      .inner
      .call(&call_opts, block_tag.0.clone())
      .await
      .map_err(NapiEyreReport)?;
    Ok(Buffer::from(output))
  }

  #[napi(ts_return_type = "Promise<bigint>")]
  pub async fn estimate_gas(&self, call_opts: &JsCallOpts) -> napi::Result<EthU256Output> {
    let call_opts = (*call_opts.0.lock().unwrap()).clone();
    let output = self
      .inner
      .estimate_gas(&call_opts)
      .await
      .map_err(NapiEyreReport)?;
    Ok(EthU256Output(U256::from(output)))
  }

  #[napi(ts_return_type = "Promise<bigint>")]
  pub async fn get_block_number(&self) -> napi::Result<EthU256Output> {
    let output = self
      .inner
      .get_block_number()
      .await
      .map_err(NapiEyreReport)?;
    Ok(EthU256Output(U256::from(output)))
  }

  #[napi]
  pub async fn get_block_by_number(
    &self,
    block_tag: &JsBlockTag,
    full_tx: bool,
  ) -> napi::Result<String> {
    let output = self
      .inner
      .get_block_by_number(block_tag.0, full_tx)
      .await
      .map_err(NapiEyreReport)?;
    Ok(
      serde_json::to_string(&output)
        .map_err(anyhow::Error::from)
        .map_err(NapiAnyhowError)?,
    )
  }

  #[napi(ts_return_type = "Promise<bigint>")]
  pub async fn chain_id(&self) -> EthU256Output {
    let output = self.inner.chain_id().await;
    EthU256Output(U256::from(output))
  }
}
