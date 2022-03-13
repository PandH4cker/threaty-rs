use crate::api::censys::models::per_page::PerPage;
use crate::api::shodan::models::dns_type::DNSType;
use crate::api::shodan::models::order::Order;
use crate::api::shodan::models::sort::Sort;
use reqwest::RequestBuilder;
use std::collections::HashMap;
use std::fmt::Display;
use std::net::IpAddr;

pub trait ShodanAPI {
    fn host_info(self, ip: IpAddr, history: Option<bool>, minify: Option<bool>) -> RequestBuilder;
    fn count_host(self, query: &str, facets: Option<&str>) -> RequestBuilder;
    fn search_host(
        self,
        query: &str,
        facets: Option<&str>,
        page: Option<PerPage<1, 100>>,
        minify: Option<bool>,
    ) -> RequestBuilder;

    fn list_facets(self) -> RequestBuilder;
    fn list_filters(self) -> RequestBuilder;
    fn into_tokens(self, query: &str) -> RequestBuilder;
    fn list_ports(self) -> RequestBuilder;
    fn list_protocols(self) -> RequestBuilder;
    fn scan(self, ips: HashMap<IpAddr, Vec<(i32, &str)>>) -> RequestBuilder;
    fn scan_internet(self, port: i32, protocol: &str) -> RequestBuilder;
    fn list_scans(self) -> RequestBuilder;
    fn scan_status(self, id: &str) -> RequestBuilder;
    fn create_alert(self, name: &str, filters: Vec<IpAddr>, expires: Option<i32>)
        -> RequestBuilder;
    fn alert_info(self, id: &str) -> RequestBuilder;
    fn delete_alert(self, id: &str) -> RequestBuilder;
    fn edit_alert(self, filters: Vec<IpAddr>) -> RequestBuilder;
    fn list_alerts(self) -> RequestBuilder;
    fn list_triggers(self) -> RequestBuilder;
    fn enable_trigger(self, id: &str, trigger: Vec<&str>) -> RequestBuilder;
    fn disable_trigger(self, id: &str, trigger: Vec<&str>) -> RequestBuilder;
    fn add_to_whitelist(self, id: &str, trigger: &str, ip: IpAddr, port: i32) -> RequestBuilder;
    fn remove_from_whitelist(
        self,
        id: &str,
        trigger: &str,
        ip: IpAddr,
        port: i32,
    ) -> RequestBuilder;

    fn add_alert_notifier(self, id: &str, notifier_id: &str) -> RequestBuilder;
    fn remove_alert_notifier(self, id: &str, notifier_id: &str) -> RequestBuilder;
    fn list_notifiers(self) -> RequestBuilder;
    fn list_notifier_providers(self) -> RequestBuilder;
    fn create_notifier_provider(
        self,
        provider: &str,
        description: &str,
        args: HashMap<&str, Box<dyn Display + 'static>>,
    ) -> RequestBuilder;

    fn delete_notifier_provider(self, id: &str) -> RequestBuilder;
    fn notifier_info(self, id: &str) -> RequestBuilder;
    fn edit_notifier(
        self,
        id: &str,
        args: HashMap<&str, Box<dyn Display + 'static>>,
    ) -> RequestBuilder;

    fn list_queries(
        self,
        page: Option<i32>,
        sort: Option<Sort>,
        order: Option<Order>,
    ) -> RequestBuilder;

    fn search_queries(self, query: &str, page: Option<i32>) -> RequestBuilder;
    fn list_tags(self, size: Option<i32>) -> RequestBuilder;
    fn list_datasets(self) -> RequestBuilder;
    fn list_dataset_files(self, dataset: &str) -> RequestBuilder;
    fn org_info(self) -> RequestBuilder;
    fn add_org_member(self, user: &str, notify: Option<bool>) -> RequestBuilder;
    fn remove_org_member(self, user: &str) -> RequestBuilder;
    fn account_profile(self) -> RequestBuilder;
    fn domain_info(
        self,
        domain: &str,
        history: Option<bool>,
        dns_type: Option<DNSType>,
        page: Option<i32>,
    ) -> RequestBuilder;

    fn dns_lookup(self, hostnames: Vec<&str>) -> RequestBuilder;
    fn reverse_dns_lookup(self, ips: Vec<IpAddr>) -> RequestBuilder;
    fn get_http_headers(self) -> RequestBuilder;
    fn whats_my_ip(self) -> RequestBuilder;
    fn api_plan_info(self) -> RequestBuilder;
}
