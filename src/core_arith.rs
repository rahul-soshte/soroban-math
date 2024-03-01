use crate::{CoreArith, SoroNum};
use soroban_sdk::{U256, I256};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_arith_operations_soronum_u32_i32() {
        let num1 = SoroNum { value: 10u32 };
        let num2 = SoroNum { value: 5u32 };
       
        assert_eq!(num1.clone().add(num2.clone()).value, 15, "Addition did not work as expected");
        assert_eq!(num1.clone().sub(SoroNum { value: 3 }).value, 7, "Subtraction did not work as expected");
        assert_eq!(num1.clone().mul(SoroNum { value: 2 }).value, 20, "Multiplication did not work as expected");
        assert_eq!(num1.clone().div(SoroNum { value: 2 }).value, 5, "Division did not work as expected");
        
        let num1 = SoroNum { value: 12_i32 };
        let num2 = SoroNum { value: 5_i32 };

        assert_eq!(num1.clone().add(num2.clone()).value, 17, "Addition did not work as expected");
        assert_eq!(num1.clone().sub(SoroNum { value: 3 }).value, 9, "Subtraction did not work as expected");
        assert_eq!(num1.clone().mul(SoroNum { value: 2 }).value, 24, "Multiplication did not work as expected");
        assert_eq!(num1.clone().div(SoroNum { value: 2 }).value, 6, "Division did not work as expected");

        
    }



}
