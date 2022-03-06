use std::net::IpAddr;
use reqwest::{RequestBuilder};
use crate::api::censys::models::{
    per_page::PerPage,
    virtual_hosts::VirtualHost
};

pub trait CensysAPI {
    fn search_hosts(self,
                    query: Option<&str>,
                    per_page: Option<PerPage<0, 100>>,
                    virtual_hosts: Option<VirtualHost>,
                    cursor: Option<&str>) -> RequestBuilder;

    fn aggregate_hosts(self,
                       query: Option<&str>,
                       field: &str,
                       num_buckets: Option<i32>,
                       virtual_hosts: Option<VirtualHost>) -> RequestBuilder;

    fn view_host(self,
                 ip: IpAddr,
                 at_time: Option<&str>) -> RequestBuilder;

    fn view_host_diff(self,
                      ip: IpAddr,
                      ip_b: Option<IpAddr>,
                      at_time: Option<&str>,
                      at_time_b: Option<&str>) -> RequestBuilder;

    fn view_host_events(self,
                        ip: IpAddr,
                        start_time: Option<&str>,
                        end_time: Option<&str>,
                        per_page: Option<PerPage<1, 50>>,
                        cursor: Option<&str>,
                        reversed: Option<bool>) -> RequestBuilder;

    fn view_host_names(self,
                       ip: IpAddr,
                       per_page: Option<PerPage<1, 1000>>,
                       cursor: Option<&str>) -> RequestBuilder;

    fn get_comments_by_host(self, ip: IpAddr) -> RequestBuilder;
    fn add_comment_by_host(self, ip: IpAddr, contents: &str) -> RequestBuilder;
    fn get_comment_by_host(self, ip: IpAddr, comment_id: &str) -> RequestBuilder;
    fn update_comment_by_host(self, ip: IpAddr, comment_id: &str, contents: &str) -> RequestBuilder;
    fn delete_comment_by_host(self, ip: IpAddr, comment_id: &str) -> RequestBuilder;
    fn get_host_metadata(self) -> RequestBuilder;
    fn list_hosts_for_tag(self, id: &str) -> RequestBuilder;
    fn get_tags_by_host(self, ip: IpAddr) -> RequestBuilder;
    fn tag_host(self, ip: IpAddr, id: &str) -> RequestBuilder;
    fn untag_host(self, ip: IpAddr, id: &str) -> RequestBuilder;
}