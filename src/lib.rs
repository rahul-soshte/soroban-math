pub struct SoroNum<T> {
    value: T,
}

impl<T> SoroNum<T> {
    // Constructor
    pub fn new(value: T) -> Self {
        Self { value }
    }
    
    // Example basic method (you'll replace this with actual arithmetic operations)
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

impl CoreArith for SoroNum<i32> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }

    fn sub(self, other: Self) -> Self {
        SoroNum { value: self.value - other.value }
    }

    fn mul(self, other: Self) -> Self {
        SoroNum { value: self.value * other.value }
    }

    fn div(self, other: Self) -> Self {
        SoroNum { value: self.value / other.value }
    }
}
