use soroban_sdk::{U256, Env};

use crate::SoroNum;

pub trait Trigonometry {
    fn sin(&self, e: &Env) -> Self;
    fn cos(&self, e: &Env) -> Self;
}

//TODO: Do proper scaled division for improvement in output
impl Trigonometry for SoroNum<U256> {
    fn sin(&self, e: &Env) -> Self {
        // Scale factor for fixed-point calculations, chosen to ensure precision
        let scale = U256::from_u128(e, 1_000_000u128);

        let radians: U256 = ((self.value.clone().mul(&scale)).mul(&U256::from_u128(e, 3_141_592))).div(&U256::from_u128(e, 180u128).mul(&scale)); 
        // Properly scale π for accurate modulo operation and convert degrees to radians if needed
        let pi_scaled = U256::from_u128(e, 3_141_592 * 2 );
        
        let x = radians.rem_euclid(&pi_scaled); // modulo 2π

        let x2 = (x.clone().mul(&x.clone())).div(&scale);
        let x3 = (x2.clone().mul(&x.clone())).div(&scale);
        let x5 = (x3.clone().mul(&x2.clone())).div(&scale);
        let x7 = (x5.clone().mul(&x2.clone())).div(&scale);

        // Calculate Taylor series terms
        // x - x^3/3! + x^5/5! - x^7/7!
        let mut result = x.clone();
        result = result.sub(&x3.div(&U256::from_u128(e, 6u128)));
        result = result.add(&x5.div(&U256::from_u128(e, 120u128)));
        result = result.sub(&x7.div(&U256::from_u128(e, 5040u128)));

        SoroNum::new(result)
    }

    fn cos(&self, e: &Env) -> Self {
        // Scale factor for fixed-point calculations
        let scale = U256::from_u128(e, 1_000_000u128);
        let radians: U256 = ((self.value.clone().mul(&scale)).mul(&U256::from_u128(e, 3_141_592))).div(&U256::from_u128(e, 180u128).mul(&scale)); 
        let pi_scaled = U256::from_u128(e, 3_141_592 * 2  );
        let x = radians.rem_euclid(&pi_scaled); // modulo 2π

        // Convert input to fixed-point representation (assume input is already scaled)
        let x2 = (x.clone().mul( &x.clone())).div(&scale);
        let x4 = (x2.clone().mul(&x2.clone())).div(&scale);
        let x6 = (x4.clone().mul(&x2.clone())).div(&scale);

        // Calculate Taylor series terms
        // 1 - x^2/2! + x^4/4! - x^6/6!
        let mut result = scale.clone();
        result = result.sub(&x2.div(&U256::from_u128(e, 2u128)));
        result = result.add(&x4.div(&U256::from_u128(e, 24u128)));
        result = result.sub(&x6.div(&U256::from_u128(e, 720u128)));

        SoroNum::new(result)
    }
}