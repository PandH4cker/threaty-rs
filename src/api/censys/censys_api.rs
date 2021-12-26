use std::future::Future;
use std::pin::Pin;
use reqwest::{Error, RequestBuilder, Response};
use crate::api::censys::models::{
    per_page::PerPage,
    virtual_hosts::VirtualHost
};

pub trait CensysAPI {
    fn search_hosts(self,
                    query: &str,
                    per_page: Option<PerPage>,
                    virtual_hosts: Option<VirtualHost>,
                    cursor: Option<&str>) -> RequestBuilder;
}