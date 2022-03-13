pub mod censys_api;
pub mod models;
pub mod censys_client;

const BASE_URL: &str = "https://search.censys.io/api";

#[derive(strum_macros::Display)]
enum Endpoints {
    #[strum(serialize = "/v2/hosts/search")]
    SearchHosts,
    #[strum(serialize = "/v2/hosts/aggregate")]
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
    UntagHost,
    #[strum(serialize = "/v1/view/certificates/{sha256}")]
    ViewCertificate,
    #[strum(serialize = "/v1/search/certificates")]
    SearchCertificates,
    #[strum(serialize = "/v1/report/certificates")]
    GenerateCertificateReport,
    #[strum(serialize = "/v1/bulk/certificates")]
    BulkCertificateLookup,
    #[strum(serialize = "/v2/certificates/{fingerprint}/hosts")]
    GetHostsByCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/comments")]
    GetCommentsByCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/comments")]
    AddCommentByCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/comments/{comment_id}")]
    GetCommentByCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/comments/{comment_id}")]
    UpdateCommentByCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/comments/{comment_id}")]
    DeleteCommentByCert,
    #[strum(serialize = "/v2/tags/{id}/certificates")]
    ListCertificatesForTag,
    #[strum(serialize = "/v2/certificates/{fingerprint}/tags")]
    GetTagsByCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/tags/{id}")]
    TagCert,
    #[strum(serialize = "/v2/certificates/{fingerprint}/tags/{id}")]
    UntagCert,
    #[strum(serialize = "/v1/data")]
    GetSeries,
    #[strum(serialize = "/v1/data/{series}")]
    ViewSeries,
    #[strum(serialize = "/v1/data/{series}/{result}")]
    ViewResult,
    #[strum(serialize = "/v1/account")]
    Account,
    #[strum(serialize = "/v2/tags")]
    ListTags,
    #[strum(serialize = "/v2/tags")]
    CreateTag,
    #[strum(serialize = "/v2/tags/{id}")]
    GetTag,
    #[strum(serialize = "/v2/tags/{id}")]
    UpdateTag,
    #[strum(serialize = "/v2/tags/{id}")]
    DeleteTag
}