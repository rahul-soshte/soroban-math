use soroban_sdk::{U256, Env, I256};

use crate::{SoroNum, CoreArith};

pub trait Sqrt {
    fn sqrt(&self, e: &Env) -> Self;
}

impl Sqrt for SoroNum<U256> {
    fn sqrt(&self, e: &Env) -> SoroNum<U256> {
        let mut low = SoroNum { value: U256::from_u32(e, 0)};
        let mut high = self.clone();
        let two = SoroNum { value: U256::from_u32(e, 2)};

        while low.clone().value < high.clone().value {
            let mid = low.clone().add(high.clone()).unwrap().div(two.clone(), e).unwrap(); // Calculate the midpoint
            let mid_squared = mid.clone().mul(mid.clone()).unwrap();

            if *mid_squared.value() == *self.value() {
                return mid; // Found exact square root
            } else if *mid_squared.value() <  *self.value() {
                low = mid.add(SoroNum {value: U256::from_u32(e, 1) }).unwrap();
            } else {
                high = mid;
            }
        }

        if *low.clone().mul(low.clone()).unwrap().value() > *self.value() {
            low = low.sub(SoroNum { value: U256::from_u32(e, 1) }).unwrap();
        }

        low
    }
}

impl Sqrt for SoroNum<I256> {

fn sqrt(&self, e: &Env) -> SoroNum<I256> {
    
    if self.value.to_i128().map_or(false, |x| x < 0) {
        return SoroNum {value: I256::from_i32(e, 0)};
    }

    let mut low = SoroNum { value: I256::from_i32(e, 0) };
    let mut high = self.clone();
    let two = SoroNum { value: I256::from_i32(e, 2) };

    while low.clone().value() < high.clone().value() {
        let mid = low.clone().add(high.clone()).unwrap().div(two.clone(), e).unwrap(); // Calculate the midpoint
        let mid_squared = mid.clone().mul(mid.clone()).unwrap();

        if *mid_squared.value() == *self.value() {
            return mid; // Found exact square root
        } else if  *mid_squared.value() < *self.value(){
            low = mid.add(SoroNum {value: I256::from_i32(e, 1)}).unwrap();
        } else {
            high = mid;
        }
    }

    // Adjust result if it's too high
    if *low.clone().mul(low.clone()).unwrap().value() > *self.value() {
        low = low.clone().sub(SoroNum {value: I256::from_i32(e, 1)}).unwrap();
    }

    low
}

}