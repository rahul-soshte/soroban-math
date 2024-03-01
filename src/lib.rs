pub mod core_arith;

#[derive(Clone, Debug)]
pub struct SoroNum<T> {
    value: T,
}

impl<T> SoroNum<T> {
    // Constructor
    pub fn new(value: T) -> Self {
        Self { value }
    }
    
    pub fn value(&self) -> &T {
        &self.value
    }
}

pub trait CoreArith {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn div(self, other: Self) -> Self;
}

