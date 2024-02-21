pub mod core_arith;


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
    //TODO: Add function signature for mul, div in the CoreArith trait 
}

