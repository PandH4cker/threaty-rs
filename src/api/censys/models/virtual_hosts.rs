#[derive(strum_macros::Display)]
pub enum VirtualHost {
    EXCLUDE,
    INCLUDE,
    ONLY,
}

impl Default for VirtualHost {
    fn default() -> Self {
        VirtualHost::EXCLUDE
    }
}