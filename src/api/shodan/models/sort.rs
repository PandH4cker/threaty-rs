#[derive(strum_macros::Display)]
pub enum Sort {
    VOTES,
    TIMESTAMP,
}

impl Default for Sort {
    fn default() -> Self {
        Sort::VOTES
    }
}
