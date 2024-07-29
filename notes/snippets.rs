// impl<const SCALE_OUT: u32> CoreArith<SCALE_OUT> for SoroNum<i128, SCALE_OUT> {
    //     type Inner = i128;
    
    //     fn add<const SCALE_IN: u32>(self, other: SoroNum<i128, SCALE_IN>) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
    //         let adjusted_other = if SCALE_OUT > SCALE_IN {
    //             other.value * 10i128.pow(SCALE_OUT - SCALE_IN)
    //         } else {
    //             other.value / 10i128.pow(SCALE_IN - SCALE_OUT)
    //         };
    
    //         self.value
    //             .checked_add(adjusted_other)
    //             .map(|value| SoroNum { value })
    //             .ok_or(ArithmeticError::Overflow)
    //     }
    
    //     fn sub<const SCALE_IN: u32>(self, other: SoroNum<i128, SCALE_IN>) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
    //         let adjusted_other = if SCALE_OUT > SCALE_IN {
    //             other.value * 10i128.pow(SCALE_OUT - SCALE_IN)
    //         } else {
    //             other.value / 10i128.pow(SCALE_IN - SCALE_OUT)
    //         };
    
    //         self.value
    //             .checked_sub(adjusted_other)
    //             .map(|value| SoroNum { value })
    //             .ok_or(ArithmeticError::Underflow)
    //     }
    
    //     fn mul<const SCALE_IN: u32>(self, other: SoroNum<i128, SCALE_IN>, env: &Env) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
    //         let result = (I256::from_i128(env, self.value).mul(&I256::from_i128(env, other.value))).div(         10i256.pow(SCALE_IN as u32)));
    //         if result > i128::MAX as i256 || result < i128::MIN as i256 {
    //             Err(ArithmeticError::Overflow)
    //         } else {
    //             Ok(SoroNum { value: result as i128 })
    //         }
    //     }
    
    //     fn div<const SCALE_IN: u32>(self, other: SoroNum<i128, SCALE_IN>, env: &Env) -> Result<SoroNum<i128, SCALE_OUT>, ArithmeticError> {
    //         if other.value == 0 {
    //             Err(ArithmeticError::DivisionByZero)
    //         } else {
    //             let result = (self.value as i256 * 10i256.pow(SCALE_OUT as u32 + SCALE_IN as u32)) / other.value as i256;
    //             if result > i128::MAX as i256 || result < i128::MIN as i256 {
    //                 Err(ArithmeticError::Overflow)
    //             } else {
    //                 Ok(SoroNum { value: result as i128 })
    //             }
    //         }
    //     }
    // }

// impl CoreArith for SoroNum<u128> {
//     fn add(self, other: Self) -> Result<Self, ArithmeticError> {
//         self.value
//             .checked_add(other.value)
//             .map(|value| SoroNum { value })
//             .ok_or(ArithmeticError::Overflow)
//     }

//     fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
//         self.value
//             .checked_sub(other.value)
//             .map(|value| SoroNum { value })
//             .ok_or(ArithmeticError::Underflow)
//     }

//     fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
//         self.value
//             .checked_mul(other.value)
//             .map(|value| SoroNum { value })
//             .ok_or(ArithmeticError::Overflow)
//     }

//     fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
//         if other.value == 0 {
//             Err(ArithmeticError::DivisionByZero)
//         } else {
//             self.value
//                 .checked_div(other.value)
//                 .map(|value| SoroNum { value })
//                 .ok_or(ArithmeticError::DivisionByZero)
//         }
//     }
// }

// impl CoreArith for SoroNum<U256> {
//     fn add(self, other: Self) -> Result<Self, ArithmeticError> {
//         Ok(SoroNum {
//             value: self.value.add(&other.value),
//         })
//     }
//     fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
//         Ok(SoroNum {
//             value: self.value.sub(&other.value),
//         })
//     }
//     fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
//         Ok(SoroNum {
//             value: self.value.mul(&other.value),
//         })
//     }
//     fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
//         if other.value == U256::from_u32(env, 0) {
//             Err(ArithmeticError::DivisionByZero)
//         } else {
//             Ok(SoroNum {
//                 value: self.value.div(&other.value),
//             })
//         }
//     }
// }

// impl CoreArith for SoroNum<I256> {
//     fn add(self, other: Self) -> Result<Self, ArithmeticError> {
//         Ok(SoroNum {
//             value: self.value.add(&other.value),
//         })
//     }
//     fn sub(self, other: Self) -> Result<Self, ArithmeticError> {
//         Ok(SoroNum {
//             value: self.value.sub(&other.value),
//         })
//     }
//     fn mul(self, other: Self) -> Result<Self, ArithmeticError> {
//         Ok(SoroNum {
//             value: self.value.mul(&other.value),
//         })
//     }
//     fn div(self, other: Self, env: &Env) -> Result<Self, ArithmeticError> {
//         if other.value == I256::from_i32(&env, 0) {
//             Err(ArithmeticError::DivisionByZero)
//         } else {
//             Ok(SoroNum {
//                 value: self.value.div(&other.value),
//             })
//         }
//     }
// }