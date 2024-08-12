use soroban_sdk::{Env, I256};
use crate::{error::ArithmeticError, SoroNum};
// Define the trait


pub trait Trigonometry {
    fn sin<const SCALE_OUT: u32>(&self, env: &Env) -> Result<Self, ArithmeticError> where Self: Sized;
    fn cos<const SCALE_OUT: u32>(&self, env: &Env) -> Result<Self, ArithmeticError> where Self: Sized;
}

impl Trigonometry for SoroNum<i128> {
    fn sin<const SCALE_OUT: u32>(&self, env: &Env) -> Result<Self, ArithmeticError> {
        let two_pi = I256::from_i128(env, 628318530717958647).mul(&I256::from_i128(env, 100));
        let input = I256::from_i128(env, self.value).mul(&I256::from_i128(env, 10).pow(20 - self.scale));
        
        // Normalize input to [0, 2π)
        let normalized = input.rem_euclid(&two_pi);
        
        // Use lookup table for common angles
        let result = match normalized.to_i128().unwrap() {
            0 => I256::from_i128(env, 0),
            104719755 => I256::from_i128(env, 500000000000000000), // π/6
            157079632 => I256::from_i128(env, 707106781186547524), // π/4
            209439510 => I256::from_i128(env, 866025403784438647), // π/3
            314159265 => I256::from_i128(env, 1000000000000000000), // π/2
            _ => {
                // Use CORDIC algorithm for other angles
                let mut x = I256::from_i128(env, 607252935008881787); // 0.6072529350088817 * 10^18
                let mut y = I256::from_i128(env, 0);
                let mut z = normalized;

                let scale_factor = I256::from_i128(env, 10).pow(18);

                for i in 0..20 {
                    let pow_2 = I256::from_i128(env, 1).shl(i);
                    let angle = I256::from_i128(env, cordic_angles(i));
                    
                    if z.gt(&I256::from_i128(env, 0)) {
                        let new_x = x.sub(&y.shr(i));
                        let new_y = y.add(&x.shr(i));
                        x = new_x;
                        y = new_y;
                        z = z.sub(&angle);
                    } else {
                        let new_x = x.add(&y.shr(i));
                        let new_y = y.sub(&x.shr(i));
                        x = new_x;
                        y = new_y;
                        z = z.add(&angle);
                    }
                }

                y
            }
        };

        let scaled_result = result.mul(&I256::from_i128(env, 10).pow(SCALE_OUT - 18));

        if scaled_result > I256::from_i128(env, i128::MAX) || scaled_result < I256::from_i128(env, i128::MIN) {
            Err(ArithmeticError::Overflow)
        } else {
            Ok(SoroNum { value: scaled_result.to_i128().unwrap(), scale: SCALE_OUT })
        }
    }

    fn cos<const SCALE_OUT: u32>(&self, env: &Env) -> Result<Self, ArithmeticError> {
        // Cosine is just sine with a phase shift of π/2
        let phase_shift = I256::from_i128(env, 314159265358979323).mul(&I256::from_i128(env, 10).pow(20 - self.scale - 18));
        let shifted = SoroNum { value: (I256::from_i128(env, self.value).add(&phase_shift)).to_i128().unwrap(), scale: self.scale };
        shifted.sin::<SCALE_OUT>(env)
    }
}

fn cordic_angles(i: u32) -> i128 {
    match i {
        0 => 785398163397448279,
        1 => 463647609000806116,
        2 => 244978663126864143,
        3 => 124354994546761438,
        4 => 62418809995957350,
        5 => 31239833430268277,
        6 => 15623728620476831,
        7 => 7812341060101111,
        8 => 3906230131966972,
        9 => 1953122516478818,
        10 => 976562189559319,
        11 => 488281211194898,
        12 => 244140620149362,
        13 => 122070311893670,
        14 => 61035156174208,
        15 => 30517578115526,
        16 => 15258789061315,
        17 => 7629394531101,
        18 => 3814697265606,
        19 => 1907348632810,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;
    use core::f64::consts;

    const SCALE: u32 = 6;
    const EPSILON: i128 = 1000; // Allow for small rounding errors

    fn assert_close(a: SoroNum<i128>, b: SoroNum<i128>) {
        assert!((a.value - b.value).abs() <= EPSILON, 
                "Expected close to {:?}, but got {:?}", b, a);
    }

    fn radian_to_soronum(radians: f64) -> SoroNum<i128> {
        SoroNum::new((radians * 1_000_000.0) as i128, SCALE)
    }

    #[test]
    fn test_sin() {
        let env = Env::default();

        // Test common angles
        assert_close(radian_to_soronum(0.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(0, SCALE));
        assert_close(radian_to_soronum(consts::PI / 6.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(500000, SCALE));
        assert_close(radian_to_soronum(consts::PI / 4.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(707107, SCALE));
        assert_close(radian_to_soronum(consts::PI / 3.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(866025, SCALE));
        assert_close(radian_to_soronum(consts::PI / 2.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(1000000, SCALE));

        // Test negative angles
        assert_close(radian_to_soronum(-consts::PI / 6.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(-500000, SCALE));

        // Test angles > 2π
        assert_close(radian_to_soronum(13.0 * consts::PI / 6.0).sin::<SCALE>(&env).unwrap(), 
                     SoroNum::new(500000, SCALE));
    }

    #[test]
    fn test_cos() {
        let env = Env::default();

        // Test common angles
        assert_close(radian_to_soronum(0.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(1000000, SCALE));
        assert_close(radian_to_soronum(consts::PI / 6.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(866025, SCALE));
        assert_close(radian_to_soronum(consts::PI / 4.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(707107, SCALE));
        assert_close(radian_to_soronum(consts::PI / 3.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(500000, SCALE));
        assert_close(radian_to_soronum(consts::PI / 2.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(0, SCALE));

        // Test negative angles
        assert_close(radian_to_soronum(-consts::PI / 6.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(866025, SCALE));

        // Test angles > 2π
        assert_close(radian_to_soronum(13.0 * consts::PI / 6.0).cos::<SCALE>(&env).unwrap(), 
                     SoroNum::new(866025, SCALE));
    }

    #[test]
    fn test_trigonometric_identities() {
        let env = Env::default();
        let angles = [0.0, consts::PI / 6.0, consts::PI / 4.0, 
                      consts::PI / 3.0, consts::PI / 2.0, 
                      consts::PI, 3.0 * consts::PI / 2.0, 2.0 * consts::PI];

        for &angle in &angles {
            let x = radian_to_soronum(angle);
            let sin_x = x.sin::<SCALE>(&env).unwrap();
            let cos_x = x.cos::<SCALE>(&env).unwrap();

            // Test sin^2(x) + cos^2(x) = 1
            let sin_squared = SoroNum::new(sin_x.value * sin_x.value / 10i128.pow(SCALE), SCALE);
            let cos_squared = SoroNum::new(cos_x.value * cos_x.value / 10i128.pow(SCALE), SCALE);
            let sum = SoroNum::new(sin_squared.value + cos_squared.value, SCALE);
            
            assert_close(sum, SoroNum::new(1000000, SCALE));
        }
    }
}