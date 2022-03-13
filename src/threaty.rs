use crate::api::censys::censys_client::CensysClient;

pub struct Threaty {
    censys_client: CensysClient,
}

impl Threaty {
    pub fn new(
        censys_api_key: &str,
        censys_secret: &str,
        user_agent: Option<String>,
        proxy: Option<String>,
    ) -> Threaty {
        Threaty {
            censys_client: CensysClient::new(censys_api_key, censys_secret, user_agent, proxy),
        }
    }

    pub fn get_censys_client(self) -> CensysClient {
        self.censys_client
    }
}
