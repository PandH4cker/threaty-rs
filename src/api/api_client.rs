use reqwest::header::HeaderName;
use reqwest::{header, Client, Proxy};
use std::collections::HashMap;
use std::time::Duration;

pub struct APIClient {
    user_agent: String,
    headers: HashMap<HeaderName, String>,
    proxy: Option<String>,
}

impl APIClient {
    pub fn new(
        user_agent: Option<String>,
        headers: Option<HashMap<HeaderName, String>>,
        proxy: Option<String>,
    ) -> APIClient {
        APIClient {
            user_agent: user_agent.unwrap_or("Threaty Client".to_string()),
            headers: headers.unwrap_or_default(),
            proxy,
        }
    }

    pub fn get_client(api_client: APIClient) -> Client {
        let mut headers = header::HeaderMap::new();

        api_client.headers.iter().for_each(|(key, value)| {
            headers.insert(key, value.parse().unwrap());
        });

        headers.insert(
            reqwest::header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );

        let mut builder = reqwest::Client::builder()
            .user_agent(api_client.user_agent)
            .default_headers(headers)
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10));

        if let Some(proxy) = api_client.proxy {
            builder = builder.proxy(Proxy::all(proxy).unwrap());
        }

        builder.build().unwrap()
    }
}
