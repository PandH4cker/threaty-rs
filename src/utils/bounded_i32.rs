pub struct BoundedI32<const LOW: i32, const HIGH: i32>(i32);

impl<const LOW: i32, const HIGH: i32> BoundedI32<{ LOW }, { HIGH }> {
    pub const LOW: i32 = LOW;
    pub const HIGH: i32 = HIGH;

    pub fn new(n: i32) -> Self {
        BoundedI32(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn fallible_new(n: i32) -> Result<Self, &'static str> {
        match n {
            n if n < Self::LOW => Err("Value too low"),
            n if n > Self::HIGH => Err("Value too high"),
            n => Ok(BoundedI32(n)),
        }
    }

    pub fn set(&mut self, n: i32) {
        *self = BoundedI32(n.min(Self::HIGH).max(Self::LOW))
    }
}

impl<const LOW: i32, const HIGH: i32> std::ops::Deref for BoundedI32<{ LOW }, { HIGH }> {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}