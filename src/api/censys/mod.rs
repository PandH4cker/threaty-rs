pub mod censys_api;
pub mod models;
pub mod censys_client;

const BASE_URL: &str = "https://search.censys.io/api";

#[derive(strum_macros::Display)]
enum Endpoints {
    #[strum(serialize = "/v2/hosts/search")]
    SearchHosts
}