#[macro_use]
extern crate dotenv_codegen;

use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use crate::api::censys::censys_client::CensysClient;
use crate::api::censys::censys_api::CensysAPI;

mod api;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let censys_client = CensysClient::new(dotenv!("CENSYS_API_KEY"),
                                      dotenv!("CENSYS_SECRET"),
                                      None,
                                      None);

    let r = censys_client.search_certificates(
        "validation.nss.valid: true",
        1,
        vec!["parsed.fingerprint_sha256"],
        false
    );
    let resp: serde_json::Value = serde_json::from_str(&*r.send().await?.text().await?)
                           .expect("Unable to parse");
    let pretty_json = serde_json::to_string_pretty(&resp);

    println!("{:}", pretty_json.unwrap());

    Ok(())
}