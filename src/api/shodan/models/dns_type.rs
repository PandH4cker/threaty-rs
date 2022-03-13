#[derive(strum_macros::Display)]
pub enum DNSType {
    A,
    AAAA,
    CNAME,
    NS,
    SOA,
    MX,
    TXT,
}

impl Default for DNSType {
    fn default() -> Self {
        DNSType::A
    }
}
