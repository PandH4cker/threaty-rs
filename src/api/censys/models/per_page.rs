use crate::utils::bounded_i32::BoundedI32;

pub struct PerPage(BoundedI32<0, 100>);

impl Default for PerPage {
    fn default() -> Self {
        PerPage { 0: BoundedI32::new(50) }
    }
}

impl ToString for PerPage {
    fn to_string(&self) -> String {
        (*self.0).to_string()
    }
}