#![deny(clippy::all)]


use napi::bindgen_prelude::*;
use napi_derive::napi;
use tokio::fs;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
async fn ensure_dir(dir:String) -> Result<()>{
  tokio::fs::create_dir_all(dir).await;
  Ok(())
}