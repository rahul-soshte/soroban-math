use soroban_sdk::{U256, I256, TryFromVal, Env, ConversionError, Val};

#[derive(Clone, Debug, Default)]
pub struct SoroNum<T> {
    value: T,
}



impl<T> SoroNum<T> {
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




// Core Math Functions
// i32
impl CoreArith for SoroNum<i32> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: &self.value + &other.value }
    }
    fn sub(self, other: Self) -> Self {
        SoroNum { value: &self.value - &other.value } 
    }

    fn mul(self, other: Self) -> Self {
        SoroNum { value: &self.value * &other.value }
    }
    fn div(self, other: Self) -> Self {
        SoroNum { value: &self.value / &other.value }
    }
}

// u32
impl CoreArith for SoroNum<u32> {
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

// u64
impl CoreArith for SoroNum<u64> {
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

// i64
impl CoreArith for SoroNum<i64> {
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

// i128
impl CoreArith for SoroNum<i128> {
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

// u128
impl CoreArith for SoroNum<u128> {
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


// U256
impl CoreArith for SoroNum<U256> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value.add(&other.value) }
    }
    fn sub(self, other: Self) -> Self {
        SoroNum { value: self.value.sub(&other.value) } 
    }
    fn mul(self, other: Self) -> Self {
        SoroNum { value: self.value.mul(&other.value) }
    }
    fn div(self, other: Self) -> Self {
        SoroNum { value: self.value.div(&other.value) }
    }
}

// I256
impl CoreArith for SoroNum<I256> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value.add(&other.value) }
    }
    fn sub(self, other: Self) -> Self {
        SoroNum { value: self.value.sub(&other.value) } 
    }
    fn mul(self, other: Self) -> Self {
        SoroNum { value: self.value.mul(&other.value) }
    }
    fn div(self, other: Self) -> Self {
        SoroNum { value: self.value.div(&other.value) }
    }
}