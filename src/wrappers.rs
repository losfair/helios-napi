use std::str::FromStr;

use ethers::types::{H160, U256};
use helios_common::types::BlockTag;
use helios_consensus::database::{ConfigDB, Database, FileDB};
use napi::bindgen_prelude::{BigInt, FromNapiValue, ToNapiValue, Uint8Array};

#[derive(Clone)]
pub enum SomeDB {
  File(FileDB),
  Config(ConfigDB),
}

impl Database for SomeDB {
  fn new(config: &helios_config::Config) -> eyre::Result<Self> {
    Ok(if config.data_dir.is_some() {
      Self::File(FileDB::new(config)?)
    } else {
      Self::Config(ConfigDB::new(config)?)
    })
  }

  fn save_checkpoint(&self, checkpoint: &[u8]) -> eyre::Result<()> {
    match self {
      Self::File(db) => db.save_checkpoint(checkpoint),
      Self::Config(db) => db.save_checkpoint(checkpoint),
    }
  }

  fn load_checkpoint(&self) -> eyre::Result<Vec<u8>> {
    match self {
      Self::File(db) => db.load_checkpoint(),
      Self::Config(db) => db.load_checkpoint(),
    }
  }
}

pub struct NapiEyreReport(pub eyre::Report);

impl From<NapiEyreReport> for anyhow::Error {
  fn from(report: NapiEyreReport) -> Self {
    anyhow::Error::msg(report.0.to_string())
  }
}

impl From<NapiEyreReport> for napi::Error {
  fn from(value: NapiEyreReport) -> Self {
    napi::Error::from_reason(&value.0.to_string())
  }
}

pub struct NapiAnyhowError(pub anyhow::Error);

impl From<NapiAnyhowError> for napi::Error {
  fn from(value: NapiAnyhowError) -> Self {
    napi::Error::from_reason(&value.0.to_string())
  }
}

#[derive(Clone, Copy, Debug)]
pub struct EthAddress(pub H160);

impl FromNapiValue for EthAddress {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    let mut ty = 0;
    napi::check_status!(
      napi::sys::napi_typeof(env, napi_val, &mut ty),
      "typeof failed"
    )?;
    match ty {
      napi::sys::ValueType::napi_string => {
        let s = String::from_napi_value(env, napi_val)?;
        let s = s.as_bytes();
        if !s.starts_with(b"0x") || s.len() != 42 {
          return Err(napi::Error::from_reason(
            "EthAddress: hex string must start with 0x and has length 42",
          ));
        }
        let mut out = [0u8; 20];
        hex::decode_to_slice(&s[2..], &mut out)
          .map_err(|_| napi::Error::from_reason("EthAddress: invalid hex string"))?;
        Ok(Self(H160(out)))
      }
      napi::sys::ValueType::napi_object => {
        let arr = Uint8Array::from_napi_value(env, napi_val)?;
        <[u8; 20]>::try_from(&arr[..])
          .map(|x| Self(H160(x)))
          .map_err(|_| napi::Error::from_reason("EthAddress: byte array must have length 20"))
      }
      _ => Err(napi::Error::from_reason("EthAddress: unknown value type")),
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct EthU256Input(pub U256);

#[derive(Clone, Copy, Debug)]
pub struct EthU256Output(pub U256);

impl FromNapiValue for EthU256Input {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> napi::Result<Self> {
    let mut ty = 0;
    napi::check_status!(
      napi::sys::napi_typeof(env, napi_val, &mut ty),
      "typeof failed"
    )?;
    match ty {
      napi::sys::ValueType::napi_string => {
        let s = String::from_napi_value(env, napi_val)?;
        if s.starts_with("0x") {
          U256::from_str(&s).map_err(|_| napi::Error::from_reason("EthU256: invalid hex string"))
        } else {
          U256::from_dec_str(&s)
            .map_err(|_| napi::Error::from_reason("EthU256: invalid decimal string"))
        }
        .map(Self)
      }
      napi::sys::ValueType::napi_bigint => {
        let bi = BigInt::from_napi_value(env, napi_val)?;
        if bi.sign_bit {
          return Err(napi::Error::from_reason("EthU256: negative number"));
        }

        let mut buf = [0u64; 4];
        let copy_len = bi.words.len().min(buf.len());
        buf[..copy_len].copy_from_slice(&bi.words[..copy_len]);
        Ok(Self(U256(buf)))
      }
      _ => Err(napi::Error::from_reason("EthU256: unknown value type")),
    }
  }
}

impl ToNapiValue for EthU256Output {
  unsafe fn to_napi_value(
    env: napi::sys::napi_env,
    val: Self,
  ) -> napi::Result<napi::sys::napi_value> {
    let bi = BigInt {
      sign_bit: false,
      words: val.0 .0.to_vec(),
    };
    BigInt::to_napi_value(env, bi)
  }
}

#[napi(js_name = "BlockTag")]
pub struct JsBlockTag(pub(crate) BlockTag);

#[napi]
impl JsBlockTag {
  #[napi(factory)]
  pub fn latest() -> Self {
    Self(BlockTag::Latest)
  }

  #[napi(factory)]
  pub fn finalized() -> Self {
    Self(BlockTag::Finalized)
  }

  #[napi(factory)]
  pub fn number(x: BigInt) -> napi::Result<Self> {
    let (signed, value, lossless) = x.get_u64();
    if signed || !lossless {
      return Err(napi::Error::from_reason(
        "BlockTag::number: input must be a u64",
      ));
    }
    Ok(Self(BlockTag::Number(value)))
  }
}
