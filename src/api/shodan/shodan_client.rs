use crate::api::api_client::APIClient;
use crate::api::censys::models::per_page::PerPage;
use crate::api::shodan::models::dns_type::DNSType;
use crate::api::shodan::models::order::Order;
use crate::api::shodan::models::sort::Sort;
use crate::api::shodan::shodan_api::ShodanAPI;
use reqwest::{Client, RequestBuilder};
use std::collections::{HashMap, LinkedList};
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
        todo!()
    }

    fn count_host(self, query: &str, facets: Option<&str>) -> RequestBuilder {
        todo!()
    }

    fn search_host(
        self,
        query: &str,
        facets: Option<&str>,
        page: Option<PerPage<1, 100>>,
        minify: Option<bool>,
    ) -> RequestBuilder {
        todo!()
    }

    fn list_facets(self) -> RequestBuilder {
        todo!()
    }

    fn list_filters(self) -> RequestBuilder {
        todo!()
    }

    fn into_tokens(self, query: &str) -> RequestBuilder {
        todo!()
    }

    fn list_ports(self) -> RequestBuilder {
        todo!()
    }

    fn list_protocols(self) -> RequestBuilder {
        todo!()
    }

    fn scan(self, ips: HashMap<IpAddr, Vec<(i32, &str)>>) -> RequestBuilder {
        todo!()
    }

    fn scan_internet(self, port: i32, protocol: &str) -> RequestBuilder {
        todo!()
    }

    fn list_scans(self) -> RequestBuilder {
        todo!()
    }

    fn scan_status(self, id: &str) -> RequestBuilder {
        todo!()
    }

    fn create_alert(
        self,
        name: &str,
        filters: Vec<IpAddr>,
        expires: Option<i32>,
    ) -> RequestBuilder {
        todo!()
    }

    fn alert_info(self, id: &str) -> RequestBuilder {
        todo!()
    }

    fn delete_alert(self, id: &str) -> RequestBuilder {
        todo!()
    }

    fn edit_alert(self, filters: Vec<IpAddr>) -> RequestBuilder {
        todo!()
    }

    fn list_alerts(self) -> RequestBuilder {
        todo!()
    }

    fn list_triggers(self) -> RequestBuilder {
        todo!()
    }

    fn enable_trigger(self, id: &str, trigger: Vec<&str>) -> RequestBuilder {
        todo!()
    }

    fn disable_trigger(self, id: &str, trigger: Vec<&str>) -> RequestBuilder {
        todo!()
    }

    fn add_to_whitelist(self, id: &str, trigger: &str, ip: IpAddr, port: i32) -> RequestBuilder {
        todo!()
    }

    fn remove_from_whitelist(
        self,
        id: &str,
        trigger: &str,
        ip: IpAddr,
        port: i32,
    ) -> RequestBuilder {
        todo!()
    }

    fn add_alert_notifier(self, id: &str, notifier_id: &str) -> RequestBuilder {
        todo!()
    }

    fn remove_alert_notifier(self, id: &str, notifier_id: &str) -> RequestBuilder {
        todo!()
    }

    fn list_notifiers(self) -> RequestBuilder {
        todo!()
    }

    fn list_notifier_providers(self) -> RequestBuilder {
        todo!()
    }

    fn create_notifier_provider(
        self,
        provider: &str,
        description: &str,
        args: HashMap<&str, Box<dyn Display + 'static>>,
    ) -> RequestBuilder {
        todo!()
    }

    fn delete_notifier_provider(self, id: &str) -> RequestBuilder {
        todo!()
    }

    fn notifier_info(self, id: &str) -> RequestBuilder {
        todo!()
    }

    fn edit_notifier(
        self,
        id: &str,
        args: HashMap<&str, Box<dyn Display + 'static>>,
    ) -> RequestBuilder {
        todo!()
    }

    fn list_queries(
        self,
        page: Option<i32>,
        sort: Option<Sort>,
        order: Option<Order>,
    ) -> RequestBuilder {
        todo!()
    }

    fn search_queries(self, query: &str, page: Option<i32>) -> RequestBuilder {
        todo!()
    }

    fn list_tags(self, size: Option<i32>) -> RequestBuilder {
        todo!()
    }

    fn list_datasets(self) -> RequestBuilder {
        todo!()
    }

    fn list_dataset_files(self, dataset: &str) -> RequestBuilder {
        todo!()
    }

    fn org_info(self) -> RequestBuilder {
        todo!()
    }

    fn add_org_member(self, user: &str, notify: Option<bool>) -> RequestBuilder {
        todo!()
    }

    fn remove_org_member(self, user: &str) -> RequestBuilder {
        todo!()
    }

    fn account_profile(self) -> RequestBuilder {
        todo!()
    }

    fn domain_info(
        self,
        domain: &str,
        history: Option<bool>,
        dns_type: Option<DNSType>,
        page: Option<i32>,
    ) -> RequestBuilder {
        todo!()
    }

    fn dns_lookup(self, hostnames: Vec<&str>) -> RequestBuilder {
        todo!()
    }

    fn reverse_dns_lookup(self, ips: Vec<IpAddr>) -> RequestBuilder {
        todo!()
    }

    fn get_http_headers(self) -> RequestBuilder {
        todo!()
    }

    fn whats_my_ip(self) -> RequestBuilder {
        todo!()
    }

    fn api_plan_info(self) -> RequestBuilder {
        todo!()
    }
}
