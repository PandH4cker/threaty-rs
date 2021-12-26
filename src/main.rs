#[macro_use]
extern crate dotenv_codegen;

use std::error::Error;
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

    let r = censys_client.search_hosts("service.service_name: HTTP",
                                                   None,
                                                   None,
                                                   None);
    let resp = r.send().await?.text().await?;

    println!("{:}", resp);

    Ok(())
}