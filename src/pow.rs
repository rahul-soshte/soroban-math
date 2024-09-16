use core::panic;

use crate::{error::ArithmeticError, CoreArith, SoroNum, SoroResult};
use soroban_sdk::Env;
use soroban_sdk::I256;

pub trait Power {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env,
    ) -> Result<Self, ArithmeticError>
    where
        Self: Sized;
}

//? Fractional Exponent, when

impl Power for SoroNum<i128> {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env,
    ) -> Result<Self, ArithmeticError> {
        
        if exponent == 0 {
            return Ok(SoroNum::new(1, SCALE_OUT));
        }

        let mut result = self.clone(); // Start with the base number
        let mut base = self.clone();
        let mut exp = exponent - 1; // Subtract 1 as we've already set result to base

        while exp > 0 {
            if exp % 2 == 1 {
                let temp = result.mul::<CALC_SCALE, CALC_SCALE>(&base, env, false)?;
                result = match temp {
                    SoroResult::I128(soronum) => soronum,
                    _ => panic!("Invalid type"),
                };
            }
            exp >>= 1;
            if exp > 0 {
                let temp = base.mul::<CALC_SCALE, CALC_SCALE>(&base, env, false)?;
                base = match temp {
                    SoroResult::I128(soronum) => soronum,
                    _ => panic!("Invalid type"),
                };
            }
        }

        // Adjust the scale of the result to SCALE_OUT
        if result.scale != SCALE_OUT {
            let scale_diff = if result.scale > SCALE_OUT {
                result.scale - SCALE_OUT
            } else {
                SCALE_OUT - result.scale
            };
            let scale_factor = 10i128.pow(scale_diff);
            
            if result.scale > SCALE_OUT {
                result.value = result.value / scale_factor;
            } else {
                result.value = result.value * scale_factor;
            }
            result.scale = SCALE_OUT;
        }

        Ok(result)
    }
}

impl Power for SoroNum<I256> {
    fn pow<const CALC_SCALE: u32, const SCALE_OUT: u32>(
        &self,
        exponent: u32,
        env: &Env,
    ) -> Result<Self, ArithmeticError> {
        let mut result = SoroNum::new(I256::from_i32(env, 1), SCALE_OUT);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                let temp = result.mul::<CALC_SCALE, SCALE_OUT>(&base, env, true)?;
                result = match temp {
                    SoroResult::I256(soronum) => soronum,
                    _ => panic!("Invalid type"),
                }
            }
            exp >>= 1;
            if exp > 0 {
                let temp = base.mul::<CALC_SCALE, SCALE_OUT>(&base, env, true)?;
                base = match temp {
                    SoroResult::I256(soronum) => soronum,
                    _ => panic!("Invalid type"),
                }
            }
        }

        Ok(result)
    }
}


#[cfg(test)]
mod tests {

    mod soronum_i128_pow_tests {
        use soroban_sdk::Env;
        use crate::{pow::Power, SoroNum};
        
        // test 1: Small Base, Small Exponent, Preciseness for 1-2 inputs
        #[test]
        fn test_pow_small_base_small_exponent() {
            let env = Env::default();

            // Case 1: When exponent is 1, it should return the same number
            let a = SoroNum::<i128>::new(1_234_567, 6); // 1.234567
            let pow_a = a.pow::<10, 6>(1, &env).unwrap();
            assert_eq!(*pow_a.value(), 1_234_567);
            assert_eq!(pow_a.scale(), 6);

            //Case 2: When exponent is arbitray number greater than 1
            let b = SoroNum::<i128>::new(1_234_567, 6); // 1.234567
            let pow_b = b.pow::<10, 7>(2, &env).unwrap();
            assert_eq!(*pow_b.value(), 1_524_1556);
            assert_eq!(pow_b.scale(), 7);

        }
        //TODO test 2: Small Base, Large Exponent, Scale is small, Precisenses for 1-2 inputs
        #[test]
        fn test_pow_small_base_large_exponent() {
            let env = Env::default();
            // test 1: Small Base, Small Exponent, Preciseness for 1-2 inputs

            // Case 1: When exponent is 25, and the base is 7.16 
            let a = SoroNum::<i128>::new(716, 2); // 7.16
            let pow_a = a.pow::<10, 6>(25, &env).unwrap();
            assert_eq!(*pow_a.value(), 2359530294415687448714390274); //TODO:  Actual value around, 2,359,530,294,415,688,700,000 
            assert_eq!(pow_a.scale(), 6);

            // Case 2: When exponent is 50, but and the base is 12.25
            //? Overflow happening, why will it overflow
            let b = SoroNum::<i128>::new(1225, 2); // 12.25
            let pow_b = b.pow::<10, 4>(50, &env).unwrap();
            assert_eq!(*pow_b.value(), 1_524_1556);
            assert_eq!(pow_b.scale(), 7);

        }

        //TODO test 3: Small/Large Base, 0 Exponent, Precisenses for 1-2 inputs
        #[test]
        fn test_pow_small_base_zero_exponent() {
            let env = Env::default();
            let a = SoroNum::<i128>::new(1_234_567, 6); // 1.234567
            let pow_a = a.pow::<10, 0>(0, &env).unwrap();
            assert_eq!(*pow_a.value(), 1);           
        }
        //TODO test 4: Large Base, Small Exponent, Precisenses for 3-4 inputs
        //TODO test 5: Large Base, Large Exponent, Precisenses for 3-4 inputs
        //TODO test 6: Overflow Handling, Large output exceeds even I256
        
    }

    mod soronum_i256_pow_tests {

        //TODO test 1: Small Base, Small Exponent, Preciseness for 3-4 inputs
        //TODO test 2: Small Base, Large Exponent, Precisenses for 3-4 inputs
        //TODO test 3: Small/Large Base, 0 Exponent, Precisenses for 3-4 inputs
        //TODO test 4: Large Base, Small Exponent, Precisenses for 3-4 inputs
        //TODO test 5: Large Base, Large Exponent, Precisenses for 3-4 inputs
        //TODO test 6: Overflow Handling, Large output exceeds even I256
        // #[test]
        // fn test_pow_positive_exponent() {
        //     todo!()
        // }
    }
}