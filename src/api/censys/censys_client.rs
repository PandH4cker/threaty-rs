use std::collections::HashMap;
use std::net::IpAddr;
use reqwest::{Client, Method, RequestBuilder, Url};
use serde_json::{Map, Number, Value};
use crate::api::api_client::APIClient;
use crate::api::censys::BASE_URL;
use crate::api::censys::censys_api::CensysAPI;
use crate::api::censys::Endpoints::{AddCommentByHost, AggregateHosts, BulkCertificateLookup, DeleteCommentByHost, GenerateCertificateReport, GetCommentByHost, GetCommentsByHost, GetHostMetadata, GetTagsByHost, ListHostsForTag, SearchCertificates, SearchHosts, TagHost, UntagHost, UpdateCommentByHost, ViewCertificate, ViewHost, ViewHostDiff, ViewHostEvents, ViewHostNames};
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
    fn search_hosts(self, query: Option<&str>, per_page: Option<PerPage<0, 100>>,
                    virtual_hosts: Option<VirtualHost>, cursor: Option<&str>) -> RequestBuilder {
        self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=SearchHosts.to_string()
                )).unwrap()
            )
            .query(&[("q", query.unwrap_or_default())])
            .query(&[("per_page", &*per_page.unwrap_or_default().to_string())])
            .query(&[("virtual_host", &*virtual_hosts.unwrap_or_default().to_string())])
            .query(&[("cursor", cursor.unwrap_or_default())])
    }

    fn aggregate_hosts(self, query: Option<&str>, field: &str, num_buckets: Option<i32>,
                       virtual_hosts: Option<VirtualHost>) -> RequestBuilder {
        self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=AggregateHosts.to_string()
                )).unwrap()
            )
            .query(&[("q", query.unwrap_or_default())])
            .query(&[("field", field)])
            .query(&[("num_buckets", num_buckets.unwrap_or(50))])
            .query(&[("virtual_hosts", &*virtual_hosts.unwrap_or_default().to_string())])
    }

    fn view_host(self, ip: IpAddr, at_time: Option<&str>) -> RequestBuilder {
        self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=ViewHost.to_string().replace("{ip}", &*ip.to_string()),
                )).unwrap()
            )
            .query(&[("at_time", at_time.unwrap_or_default())])
    }

    fn view_host_diff(self,
                      ip: IpAddr,
                      ip_b: Option<IpAddr>,
                      at_time: Option<&str>,
                      at_time_b: Option<&str>) -> RequestBuilder {
        let req = self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=ViewHostDiff.to_string().replace("{ip}", &*ip.to_string()),
                )).unwrap()
            )
            .query(&[("at_time", at_time.unwrap_or_default())])
            .query(&[("at_time_b", at_time_b.unwrap_or_default())]);

        if let Some(unwrapped) = ip_b {
            return req.query(&[("ip_b", unwrapped)]);
        } req
    }

    fn view_host_events(self,
                        ip: IpAddr,
                        start_time: Option<&str>,
                        end_time: Option<&str>,
                        per_page: Option<PerPage<1, 50>>,
                        cursor: Option<&str>,
                        reversed: Option<bool>) -> RequestBuilder {
        self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=ViewHostEvents.to_string().replace("{ip}", &*ip.to_string())
                )).unwrap()
            )
            .query(&[("start_time", start_time.unwrap_or_default())])
            .query(&[("end_time", end_time.unwrap_or_default())])
            .query(&[("per_page", &*per_page.unwrap_or_default().to_string())])
            .query(&[("cursor", cursor.unwrap_or_default())])
            .query(&[("reversed", reversed.unwrap_or_default())])
    }

    fn view_host_names(self, ip: IpAddr, per_page: Option<PerPage<1, 1000>>, cursor: Option<&str>) -> RequestBuilder {
        self.client
            .request(Method::GET, Url::parse(
                &*format!(
                    "{base}{endpoint}",
                    base=BASE_URL,
                    endpoint=ViewHostNames.to_string().replace("{ip}", &*ip.to_string())

                )).unwrap()
            )
            .query(&[("per_page", &*per_page.unwrap_or_default().to_string())])
            .query(&[("cursor", cursor.unwrap_or_default())])
    }

    fn get_comments_by_host(self, ip: IpAddr) -> RequestBuilder {
        self.client.request(Method::GET, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=GetCommentsByHost.to_string().replace("{ip}", &*ip.to_string())
            )).unwrap()
        )
    }

    fn add_comment_by_host(self, ip: IpAddr, contents: &str) -> RequestBuilder {
        let mut json_body = Map::new();
        json_body.insert("contents".to_string(), Value::String(contents.to_string()));

        self.client.request(Method::POST, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=AddCommentByHost.to_string().replace("{ip}", &*ip.to_string())
            )).unwrap()
        )
        .json(&json_body)
    }

    fn get_comment_by_host(self, ip: IpAddr, comment_id: &str) -> RequestBuilder {
        self.client.request(Method::GET, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=GetCommentByHost.to_string()
                                         .replace("{ip}", &*ip.to_string())
                                         .replace("{comment_id}", &*comment_id.to_string())
            )).unwrap()
        )
    }

    fn update_comment_by_host(self, ip: IpAddr, comment_id: &str, contents: &str) -> RequestBuilder {
        let mut json_body = Map::new();
        json_body.insert("contents".to_string(), Value::String(contents.to_string()));

        self.client.request(Method::PUT, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=UpdateCommentByHost.to_string()
                    .replace("{ip}", &*ip.to_string())
                    .replace("{comment_id}", &*comment_id.to_string())
            )).unwrap()
        )
        .json(&json_body)
    }

    fn delete_comment_by_host(self, ip: IpAddr, comment_id: &str) -> RequestBuilder {
        self.client.request(Method::DELETE, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=DeleteCommentByHost.to_string()
                    .replace("{ip}", &*ip.to_string())
                    .replace("{comment_id}", &*comment_id.to_string())
            )).unwrap()
        )
    }

    fn get_host_metadata(self) -> RequestBuilder {
        self.client.request(Method::GET, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=GetHostMetadata.to_string()
            )).unwrap()
        )
    }

    fn list_hosts_for_tag(self, id: &str) -> RequestBuilder {
        self.client.request(Method::GET, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=ListHostsForTag.to_string().replace("{id}", &*id.to_string())
            )).unwrap()
        )
    }

    fn get_tags_by_host(self, ip: IpAddr) -> RequestBuilder {
        self.client.request(Method::GET, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=GetTagsByHost.to_string().replace("{ip}", &*ip.to_string())
            )).unwrap()
        )
    }

    fn tag_host(self, ip: IpAddr, id: &str) -> RequestBuilder {
        self.client.request(Method::PUT, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=TagHost.to_string()
                                .replace("{ip}", &*ip.to_string())
                                .replace("{id}", id)
            )).unwrap()
        )
    }

    fn untag_host(self, ip: IpAddr, id: &str) -> RequestBuilder {
        self.client.request(Method::DELETE, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=UntagHost.to_string()
                                  .replace("{ip}", &*ip.to_string())
                                  .replace("{id}", id)
            )).unwrap()
        )
    }

    fn view_certificate(self, sha256: &str) -> RequestBuilder {
        self.client.request(Method::GET, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=ViewCertificate.to_string()
                    .replace("{sha256}", &*sha256.to_string())
            )).unwrap()
        )
    }

    fn search_certificates(self,
                           query: &str,
                           page: i32,
                           fields: Vec<&str>,
                           flatten: bool) -> RequestBuilder {
        let mut json_body = Map::new();
        json_body.insert("query".to_string(), Value::String(query.to_string()));
        json_body.insert("page".to_string(), Value::Number(Number::from(page)));
        json_body.insert("fields".to_string(), Value::Array(
            fields.iter()
                  .map(|f| Value::String(f.to_string()))
                  .collect::<Vec<Value>>()
        ));
        json_body.insert("flatten".to_string(), Value::Bool(flatten));

        self.client.request(Method::POST, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=SearchCertificates.to_string()
            )).unwrap()
        ).json(&json_body)
    }

    fn generate_certificate_report(self, query: &str, field: &str, bucket: i32) -> RequestBuilder {
        let mut json_body = Map::new();
        json_body.insert("query".to_string(), Value::String(query.to_string()));
        json_body.insert("fields".to_string(), Value::String(field.to_string()));
        json_body.insert("bucket".to_string(), Value::Number(Number::from(bucket)));

        self.client.request(Method::POST, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=GenerateCertificateReport.to_string()
            )).unwrap()
        ).json(&json_body)
    }

    fn bulk_certificate_lookup(self, fingerprints: Vec<&str>) -> RequestBuilder {
        let mut json_body = Map::new();
        json_body.insert("fingerprints".to_string(), Value::Array(
            fingerprints.iter()
                        .map(|f| Value::String(f.to_string()))
                        .collect::<Vec<Value>>()
        ));

        self.client.request(Method::POST, Url::parse(
            &*format!(
                "{base}{endpoint}",
                base=BASE_URL,
                endpoint=BulkCertificateLookup.to_string()
            )).unwrap()
        ).json(&json_body)
    }
}