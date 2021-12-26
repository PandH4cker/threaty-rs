struct Threaty {
    censys_api_key: String,
    censys_secret: String
}

impl Threaty {
    pub fn new(censys_api_key: String, censys_secret: String) -> Threaty {
        Threaty { censys_api_key, censys_secret }
    }
}