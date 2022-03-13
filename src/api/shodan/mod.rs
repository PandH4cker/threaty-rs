mod models;
pub mod shodan_api;
pub mod shodan_client;

const BASE_URL: &str = "https://api.shodan.io";

#[derive(strum_macros::Display)]
enum Endpoints {
    #[strum(serialize = "/shodan/host/{ip}")]
    HostInfo,
    #[strum(serialize = "/shodan/host/count")]
    CountHost,
    #[strum(serialize = "/shodan/host/search")]
    SearchHost,
    #[strum(serialize = "/shodan/host/search/facets")]
    ListFacets,
    #[strum(serialize = "/shodan/host/search/filters")]
    ListFilters,
    #[strum(serialize = "/shodan/host/search/tokens")]
    IntoTokens,
    #[strum(serialize = "/shodan/ports")]
    ListPorts,
    #[strum(serialize = "/shodan/protocols")]
    ListProtocols,
    #[strum(serialize = "/shodan/scan")]
    Scan,
    #[strum(serialize = "/shodan/scan/internet")]
    ScanInternet,
    #[strum(serialize = "/shodan/scans")]
    ListScans,
    #[strum(serialize = "/shodan/scan/{id}")]
    ScanStatus,
    #[strum(serialize = "/shodan/alert")]
    CreateAlert,
    #[strum(serialize = "/shodan/alert/{id}/info")]
    AlertInfo,
    #[strum(serialize = "/shodan/alert/{id}")]
    DeleteAlert,
    #[strum(serialize = "/shodan/alert/{id}")]
    EditAlert,
    #[strum(serialize = "/shodan/alert/info")]
    ListAlerts,
    #[strum(serialize = "/shodan/alert/triggers")]
    ListTriggers,
    #[strum(serialize = "/shodan/alert/{id}/trigger/{trigger}")]
    EnableTrigger,
    #[strum(serialize = "/shodan/alert/{id}/trigger/{trigger}")]
    DisableTrigger,
    #[strum(serialize = "/shodan/alert/{id}/trigger/{trigger}/ignore/{service}")]
    AddToWhitelist,
    #[strum(serialize = "/shodan/alert/{id}/trigger/{trigger}/ignore/{service}")]
    RemoveFromWhitelist,
    #[strum(serialize = "/shodan/alert/{id}/notifier/{notifier_id}")]
    AddAlertNotifier,
    #[strum(serialize = "/shodan/alert/{id}/notifier/{notifier_id}")]
    RemoveAlertNotifier,
    #[strum(serialize = "/notifier")]
    ListNotifiers,
    #[strum(serialize = "/notifier/provider")]
    ListNotifierProviders,
    #[strum(serialize = "/notifier")]
    CreateNotifierProvider,
    #[strum(serialize = "/notifier/{id}")]
    DeleteNotifierProvider,
    #[strum(serialize = "/notifier/{id}")]
    NotifierInfo,
    #[strum(serialize = "/notifier/{id}")]
    EditNotifier,
    #[strum(serialize = "/shodan/query")]
    ListQueries,
    #[strum(serialize = "/shodan/query/search")]
    SearchQueries,
    #[strum(serialize = "/shodan/query/tags")]
    ListTags,
    #[strum(serialize = "/shodan/data")]
    ListDatasets,
    #[strum(serialize = "/shodan/data/{dataset}")]
    ListDatasetFiles,
    #[strum(serialize = "/org")]
    OrgInfo,
    #[strum(serialize = "/org/member/{user}")]
    AddOrgMember,
    #[strum(serialize = "/org/member/{user}")]
    RemoveOrgMember,
    #[strum(serialize = "/account/profile")]
    AccountProfile,
    #[strum(serialize = "/dns/domain/{domain}")]
    DomainInfo,
    #[strum(serialize = "/dns/resolve")]
    DNSLookup,
    #[strum(serialize = "/dns/reverse")]
    ReverseDNSLookup,
    #[strum(serialize = "/tools/httpheaders")]
    GetHttpHeaders,
    #[strum(serialize = "/tools/myip")]
    WhatsMyIp,
    #[strum(serialize = "/api-info")]
    APIPlanInfo,
}
