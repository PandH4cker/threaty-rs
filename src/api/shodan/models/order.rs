#[derive(strum_macros::Display)]
pub enum Order {
    ASC,
    DESC,
}

impl Default for Order {
    fn default() -> Self {
        Order::ASC
    }
}
