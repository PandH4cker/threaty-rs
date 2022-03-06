use crate::utils::bounded_i32::BoundedI32;

pub struct PerPage<const LOW: i32, const HIGH: i32>(BoundedI32<LOW, HIGH>);

impl<const LOW: i32, const HIGH: i32> Default for PerPage<{ LOW }, { HIGH }> {
    fn default() -> Self {
        PerPage { 0: BoundedI32::new(HIGH / 2) }
    }
}

impl<const LOW: i32, const HIGH: i32> ToString for PerPage<{ LOW }, { HIGH }> {
    fn to_string(&self) -> String {
        (*self.0).to_string()
    }
}