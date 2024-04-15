#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::fs::DirEntry;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
async fn ensure_dir(dir: String) -> Result<()> {
  tokio::fs::create_dir_all(dir).await;
  Ok(())
}

#[napi]
async fn empty_dir(dir: String) -> Result<()> {
  let items = std::fs::read_dir(dir.clone());
  match items {
    Ok(items) => {
      for item in items.into_iter() {
        let entry = DirEntry::from(item.unwrap());
        let path = entry.path();
        let file_type = entry.file_type();
        if file_type.unwrap().is_dir() {
          tokio::fs::remove_dir_all(path).await.unwrap();
        } else {
          
          tokio::fs::remove_file(path).await?;
        }
      }
    }
    Err(err) => {
      tokio::fs::create_dir_all(dir).await.unwrap();
    }
  }
  Ok(())
}
