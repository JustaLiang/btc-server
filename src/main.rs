use std::time::Duration;
use reqwest::{Client, header};
use serde_json::Value;
mod btc_monitor;

const RPC_URL: &str = "https://docs-demo.btc.quiknode.pro/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut latest_block_hash = String::new();
  let mut headers = header::HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  let client = Client::new();

  while latest_block_hash != "..." {
    let data = client.post(RPC_URL)
      .headers(headers.clone())
      .body("{ \"method\": \"getbestblockhash\" }")
      .send()
      .await?
      .text()
      .await?;
    let data: Value = serde_json::from_str(&data).unwrap();
    let block_hash = data.get("result").unwrap().to_string();
  
    if block_hash != latest_block_hash {
      latest_block_hash = block_hash;
      let req_body = format!("{{\"method\": \"getblockheader\", \"params\": [{}]}}", latest_block_hash);
      // println!("{}", &req_body);
      let data = client.post(RPC_URL)
        .headers(headers.clone())
        .body(req_body)
        .send()
        .await?
        .text()
        .await?;
      let data: Value = serde_json::from_str(&data).unwrap();
      println!("{:?}", &data);
    };

    /*
      Object {"error": Null, "id": Null, "result": Object {"bits": String("1703e8b3"), "chainwork": String("000000000000000000000000000000000000000063079d11970d7edc05174d00"), "confirmations": Number(1), "difficulty": Number(72006146478567.1), "hash": String("00000000000000000001c59446f1c751e9e37b720aed78091bbf48f998ded77d"), "height": Number(823260), "mediantime": Number(1703759184), "merkleroot": String("f1d08a64bc62129e5a2c330dd875264bd80c9c275fd00862bd30c81ea1390a30"), "nTx": Number(3324), "nonce": Number(1094862165), "previousblockhash": String("000000000000000000007963cb96014ba2852320255ef5d6a5591b2e69ca5807"), "time": Number(1703760620), "version": Number(538378240), "versionHex": String("20170000")}}
     */

    tokio::time::sleep(Duration::from_secs(30)).await
  }

  Ok(())
}
