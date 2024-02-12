use crate::{CoreArith, SoroNum};
use soroban_sdk::{U256, I256};

// i32
impl CoreArith for SoroNum<i32> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }
}

//u32
impl CoreArith for SoroNum<u32> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }
}


// u64
impl CoreArith for SoroNum<u64> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }
}

// i64
impl CoreArith for SoroNum<i64> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }
}

// i128
impl CoreArith for SoroNum<i128> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }
}

// u128
impl CoreArith for SoroNum<u128> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value + other.value }
    }
}

// U256
impl CoreArith for SoroNum<U256> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value.add(&other.value) }
    }
}

// I256
impl CoreArith for SoroNum<I256> {
    fn add(self, other: Self) -> Self {
        SoroNum { value: self.value.add(&other.value) }
    }
}
