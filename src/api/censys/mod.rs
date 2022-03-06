pub mod censys_api;
pub mod models;
pub mod censys_client;

const BASE_URL: &str = "https://search.censys.io/api";

#[derive(strum_macros::Display)]
enum Endpoints {
    #[strum(serialize = "/v2/hosts/search")]
    SearchHosts,
    #[strum(serialize = "/v2/hosts/aggregate")] // TODO
    AggregateHosts,
    #[strum(serialize = "/v2/hosts/{ip}")]
    ViewHost,
    #[strum(serialize = "/v2/hosts/{ip}/diff")]
    ViewHostDiff,
    #[strum(serialize = "/v2/experimental/hosts/{ip}/events")]
    ViewHostEvents,
    #[strum(serialize = "/v2/hosts/{ip}/names")]
    ViewHostNames,
    #[strum(serialize = "/v2/hosts/{ip}/comments")]
    GetCommentsByHost,
    #[strum(serialize = "/v2/hosts/{ip}/comments")]
    AddCommentByHost,
    #[strum(serialize = "/v2/hosts/{ip}/comments/{comment_id}")]
    GetCommentByHost,
    #[strum(serialize = "/v2/hosts/{ip}/comments/{comment_id}")]
    UpdateCommentByHost,
    #[strum(serialize = "/v2/hosts/{ip}/comments/{comment_id}")]
    DeleteCommentByHost,
    #[strum(serialize = "/v2/metadata/hosts")]
    GetHostMetadata,
    #[strum(serialize = "/v2/tags/{id}/hosts")]
    ListHostsForTag,
    #[strum(serialize = "/v2/hosts/{ip}/tags")]
    GetTagsByHost,
    #[strum(serialize = "/v2/hosts/{ip}/tags/{id}")]
    TagHost,
    #[strum(serialize = "/v2/hosts/{ip}/tags/{id}")]
    UntagHost
}