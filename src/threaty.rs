use crate::api::censys::censys_client::CensysClient;
use crate::api::shodan::shodan_client::ShodanClient;

pub struct Threaty {
    censys_client: CensysClient,
    shodan_client: ShodanClient,
}

impl Threaty {
    pub fn new(
        censys_api_key: &str,
        censys_secret: &str,
        shodan_api_key: &str,
        user_agent: Option<String>,
        proxy: Option<String>,
    ) -> Threaty {
        Threaty {
            censys_client: CensysClient::new(
                censys_api_key,
                censys_secret,
                user_agent.clone(),
                proxy.clone(),
            ),
            shodan_client: ShodanClient::new(shodan_api_key, user_agent, proxy),
        }
    }

    pub fn get_censys_client(self) -> CensysClient {
        self.censys_client
    }
}
