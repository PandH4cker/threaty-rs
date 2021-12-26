use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use reqwest::{Client, Error, Method, RequestBuilder, Response, Url};
use crate::api::api_client::APIClient;
use crate::api::censys::BASE_URL;
use crate::api::censys::censys_api::CensysAPI;
use crate::api::censys::Endpoints::SearchHosts;
use crate::api::censys::models::per_page::PerPage;
use crate::api::censys::models::virtual_hosts::VirtualHost;

pub struct CensysClient {
    client: Client
}

impl CensysClient {
    pub fn new(censys_api_key: &str, censys_secret: &str, user_agent: Option<String>, proxy: Option<String>) -> CensysClient {
        let basic_auth = [
            "Basic",
            base64::encode(format!("{}:{}", censys_api_key, censys_secret)).as_str()
        ].join(" ");

        let mut headers = HashMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, basic_auth.to_string());

        CensysClient {
            client: APIClient::get_client(APIClient::new(
                user_agent,
                Option::from(headers),
                proxy
            ))
        }
    }
}

impl CensysAPI for CensysClient {
    fn search_hosts(self, query: &str,
                    per_page: Option<PerPage>,
                    virtual_hosts: Option<VirtualHost>,
                    cursor: Option<&str>) -> RequestBuilder {
        self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=SearchHosts.to_string()
                )).unwrap()
            )
            .query(&[("q", query)])
            .query(&[("per_page", &*per_page.unwrap_or_default().to_string())])
            .query(&[("virtual_host", &*virtual_hosts.unwrap_or_default().to_string())])
            .query(&[("cursor", cursor.unwrap_or_default())])
    }
}