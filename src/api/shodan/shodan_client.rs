use crate::api::api_client::APIClient;
use crate::api::censys::models::per_page::PerPage;
use crate::api::shodan::models::dns_type::DNSType;
use crate::api::shodan::models::order::Order;
use crate::api::shodan::models::sort::Sort;
use crate::api::shodan::shodan_api::ShodanAPI;
use crate::api::shodan::Endpoints::{
    APIPlanInfo, AccountProfile, AddAlertNotifier, AddOrgMember, AddToWhitelist, AlertInfo,
    CountHost, CreateAlert, CreateNotifierProvider, DNSLookup, DeleteAlert, DeleteNotifierProvider,
    DisableTrigger, DomainInfo, EditAlert, EditNotifier, EnableTrigger, GetHttpHeaders, HostInfo,
    IntoTokens, ListAlerts, ListDatasetFiles, ListDatasets, ListFacets, ListFilters,
    ListNotifierProviders, ListNotifiers, ListPorts, ListProtocols, ListQueries, ListScans,
    ListTags, ListTriggers, NotifierInfo, OrgInfo, RemoveAlertNotifier, RemoveFromWhitelist,
    RemoveOrgMember, ReverseDNSLookup, Scan, ScanInternet, ScanStatus, SearchHost, SearchQueries,
    WhatsMyIp,
};
use crate::api::shodan::BASE_URL;
use reqwest::{Client, Method, RequestBuilder, Url};
use serde_json::{Map, Number, Value};
use std::collections::HashMap;
use std::fmt::Display;

use std::net::IpAddr;

pub struct ShodanClient {
    client: Client,
    api_key: String,
}

impl ShodanClient {
    pub fn new(
        shodan_api_key: &str,
        user_agent: Option<String>,
        proxy: Option<String>,
    ) -> ShodanClient {
        ShodanClient {
            client: APIClient::get_client(APIClient::new(user_agent, None, proxy)),
            api_key: shodan_api_key.to_string(),
        }
    }
}

impl ShodanAPI for ShodanClient {
    fn host_info(self, ip: IpAddr, history: Option<bool>, minify: Option<bool>) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = HostInfo.to_string().replace("{ip}", &*ip.to_string())
                ))
                .unwrap(),
            )
            .query(&[("history", history.unwrap_or_default())])
            .query(&[("minify", minify.unwrap_or_default())])
            .query(&[("key", self.api_key)])
    }

    fn count_host(self, query: &str, facets: Option<&str>) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = CountHost.to_string()
                ))
                .unwrap(),
            )
            .query(&[("query", query)])
            .query(&[("facets", facets.unwrap_or_default())])
            .query(&[("key", self.api_key)])
    }

    fn search_host(
        self,
        query: &str,
        facets: Option<&str>,
        page: Option<PerPage<1, 100>>,
        minify: Option<bool>,
    ) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = SearchHost.to_string()
                ))
                .unwrap(),
            )
            .query(&[("query", query)])
            .query(&[("facets", facets.unwrap_or_default())])
            .query(&[("page", &*page.unwrap_or_default().to_string())])
            .query(&[("minify", minify.unwrap_or_default())])
            .query(&[("key", self.api_key)])
    }

    fn list_facets(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListFacets.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn list_filters(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListFilters.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn into_tokens(self, query: &str) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = IntoTokens.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("query", query)])
    }

    fn list_ports(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListPorts.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn list_protocols(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListProtocols.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn scan(self, ips: HashMap<IpAddr, Vec<(i32, &str)>>) -> RequestBuilder {
        let mut json_body = Map::new();
        json_body.insert(
            "ips".to_string(),
            Value::Object(
                ips.iter()
                    .map(|(k, v)| {
                        (
                            k.to_string(),
                            Value::Array(
                                v.iter()
                                    .map(|(port, service)| {
                                        Value::Array(vec![
                                            Value::String(port.to_string()),
                                            Value::String(service.to_string()),
                                        ])
                                    })
                                    .collect::<Vec<Value>>(),
                            ),
                        )
                    })
                    .collect::<Map<String, Value>>(),
            ),
        );

        self.client
            .request(
                Method::POST,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = Scan.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .json(&json_body)
    }

    fn scan_internet(self, port: i32, protocol: &str) -> RequestBuilder {
        self.client
            .request(
                Method::POST,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ScanInternet.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("port", port)])
            .query(&[("protocol", protocol)])
    }

    fn list_scans(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListScans.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn scan_status(self, id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ScanStatus.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&["id", id])
    }

    fn create_alert(
        self,
        name: &str,
        filters: Vec<IpAddr>,
        expires: Option<i32>,
    ) -> RequestBuilder {
        let mut filters_map = Map::new();
        filters_map.insert(
            "ip".to_string(),
            Value::Array(
                filters
                    .iter()
                    .map(|ip| Value::String(ip.to_string()))
                    .collect::<Vec<Value>>(),
            ),
        );

        let mut json_body = Map::new();
        json_body.insert("name".to_string(), Value::String(name.to_string()));
        json_body.insert("filters".to_string(), Value::Object(filters_map));
        if let Some(expires) = expires {
            json_body.insert("expires".to_string(), Value::Number(Number::from(expires)));
        }

        self.client
            .request(
                Method::POST,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = CreateAlert.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .json(&json_body)
    }

    fn alert_info(self, id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = AlertInfo.to_string().replace("{id}", id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn delete_alert(self, id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::DELETE,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = DeleteAlert.to_string().replace("{id}", id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn edit_alert(self, id: &str, filters: Vec<IpAddr>) -> RequestBuilder {
        let mut filters_map = Map::new();
        filters_map.insert(
            "ip".to_string(),
            Value::Array(
                filters
                    .iter()
                    .map(|ip| Value::String(ip.to_string()))
                    .collect::<Vec<Value>>(),
            ),
        );

        let mut json_body = Map::new();
        json_body.insert("filters".to_string(), Value::Object(filters_map));
        self.client
            .request(
                Method::POST,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = EditAlert.to_string().replace("{id}", id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .json(&json_body)
    }

    fn list_alerts(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListAlerts.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn list_triggers(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListTriggers.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn enable_trigger(self, id: &str, trigger: Vec<&str>) -> RequestBuilder {
        self.client
            .request(
                Method::PUT,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = EnableTrigger
                        .to_string()
                        .replace("{id}", id)
                        .replace("{trigger}", &*trigger.join(","))
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn disable_trigger(self, id: &str, trigger: Vec<&str>) -> RequestBuilder {
        self.client
            .request(
                Method::DELETE,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = DisableTrigger
                        .to_string()
                        .replace("{id}", id)
                        .replace("{trigger}", &*trigger.join(","))
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn add_to_whitelist(self, id: &str, trigger: &str, ip: IpAddr, port: i32) -> RequestBuilder {
        self.client
            .request(
                Method::PUT,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = AddToWhitelist
                        .to_string()
                        .replace("{id}", id)
                        .replace("{trigger}", trigger)
                        .replace("{service}", &*format!("{ip}:{port}", ip = ip, port = port))
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn remove_from_whitelist(
        self,
        id: &str,
        trigger: &str,
        ip: IpAddr,
        port: i32,
    ) -> RequestBuilder {
        self.client
            .request(
                Method::DELETE,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = RemoveFromWhitelist
                        .to_string()
                        .replace("{id}", id)
                        .replace("{trigger}", trigger)
                        .replace("{service}", &*format!("{ip}:{port}", ip = ip, port = port))
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn add_alert_notifier(self, id: &str, notifier_id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::PUT,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = AddAlertNotifier
                        .to_string()
                        .replace("{id}", id)
                        .replace("{notifier_id}", notifier_id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn remove_alert_notifier(self, id: &str, notifier_id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::DELETE,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = RemoveAlertNotifier
                        .to_string()
                        .replace("{id}", id)
                        .replace("{notifier_id}", notifier_id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn list_notifiers(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListNotifiers.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn list_notifier_providers(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListNotifierProviders.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn create_notifier_provider(
        self,
        provider: &str,
        description: &str,
        args: HashMap<&str, Box<dyn Display + 'static>>,
    ) -> RequestBuilder {
        self.client
            .request(
                Method::POST,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = CreateNotifierProvider.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .body(format!(
                "provider={provider}&description={description}&{args}",
                provider = provider,
                description = description,
                args = args
                    .iter()
                    .map(|(k, v)| format!("{key}={value}", key = k, value = v))
                    .collect::<Vec<String>>()
                    .join("&")
            ))
    }

    fn delete_notifier_provider(self, id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::DELETE,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = DeleteNotifierProvider.to_string().replace("{id}", id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn notifier_info(self, id: &str) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = NotifierInfo.to_string().replace("{id}", id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn edit_notifier(
        self,
        id: &str,
        args: HashMap<&str, Box<dyn Display + 'static>>,
    ) -> RequestBuilder {
        self.client
            .request(
                Method::PUT,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = EditNotifier.to_string().replace("{id}", id)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .body(format!(
                "{args}",
                args = args
                    .iter()
                    .map(|(k, v)| format!("{key}={value}", key = k, value = v))
                    .collect::<Vec<String>>()
                    .join("&")
            ))
    }

    fn list_queries(
        self,
        page: Option<i32>,
        sort: Option<Sort>,
        order: Option<Order>,
    ) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListQueries.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("page", page.unwrap_or(1))])
            .query(&[("sort", &*sort.unwrap_or_default().to_string())])
            .query(&[("order", &*order.unwrap_or_default().to_string())])
    }

    fn search_queries(self, query: &str, page: Option<i32>) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = SearchQueries.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("query", query)])
            .query(&[("page", page.unwrap_or(1))])
    }

    fn list_tags(self, size: Option<i32>) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListTags.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("size", size.unwrap_or(10))])
    }

    fn list_datasets(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListDatasets.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn list_dataset_files(self, dataset: &str) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ListDatasetFiles.to_string().replace("{dataset}", dataset)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn org_info(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = OrgInfo.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn add_org_member(self, user: &str, notify: Option<bool>) -> RequestBuilder {
        self.client
            .request(
                Method::PUT,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = AddOrgMember.to_string().replace("{user}", user)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("notify", notify.unwrap_or_default())])
    }

    fn remove_org_member(self, user: &str) -> RequestBuilder {
        self.client
            .request(
                Method::DELETE,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = RemoveOrgMember.to_string().replace("{user}", user)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn account_profile(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = AccountProfile.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn domain_info(
        self,
        domain: &str,
        history: Option<bool>,
        dns_type: Option<DNSType>,
        page: Option<i32>,
    ) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = DomainInfo.to_string().replace("{domain}", domain)
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("history", history.unwrap_or_default())])
            .query(&[("type", &*dns_type.unwrap_or_default().to_string())])
            .query(&[("page", page.unwrap_or(1))])
    }

    fn dns_lookup(self, hostnames: Vec<&str>) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = DNSLookup.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[("hostnames", hostnames.join(","))])
    }

    fn reverse_dns_lookup(self, ips: Vec<IpAddr>) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = ReverseDNSLookup.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
            .query(&[(
                "ips",
                ips.iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            )])
    }

    fn get_http_headers(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = GetHttpHeaders.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn whats_my_ip(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = WhatsMyIp.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }

    fn api_plan_info(self) -> RequestBuilder {
        self.client
            .request(
                Method::GET,
                Url::parse(&*format!(
                    "{base}{endpoint}",
                    base = BASE_URL,
                    endpoint = APIPlanInfo.to_string()
                ))
                .unwrap(),
            )
            .query(&[("key", self.api_key)])
    }
}
