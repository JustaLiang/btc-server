#![allow(dead_code)]

use reqwest::{Client, header::{self, HeaderMap}, Error};
use serde_json::{Value, Map};

struct BtcMonitor {
  client: Client,
  headers: HeaderMap,
  rpc_url: String,
  polling_interval: u64,
  latest_block_hash: String,
  latest_block_header: Value,
}

impl BtcMonitor {

  pub fn new(rpc_url: String, polling_interval: u64) -> Self {
    let client = Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let latest_block_hash = String::new();
    let latest_block_header = Value::Object(Map::new());

    BtcMonitor {
      client,
      headers,
      rpc_url,
      polling_interval,
      latest_block_hash,
      latest_block_header,
    }
  }

  pub async fn update_block_header() -> Result<Value, Error> {
    Ok(Value::from("{}"))
  }
}
