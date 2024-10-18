use soroban_sdk::{ I256, U256};

// Trait for U256 to I256 conversion
pub trait U256ToI256 {
    // Convert U256 to I256
    fn to_i256(&self) -> Option<I256>;
}

// Trait for I256 to U256 conversion
pub trait I256ToU256 {
    // Convert U256 to I256
    fn to_u256(&self) -> Option<U256>;

    // Get the absolute value of I256 as U256
    fn abs_u256(&self) -> U256;
}

// Implement U256ToI256 for U256
impl U256ToI256 for U256 {
    fn to_i256(&self) -> Option<I256> {
        // Maximum I256 value
        let max_i256 = U256::from_parts(&self.env(), 0x7fffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff);
        
        // If the U256 is greater than max I256 return a None
        if *self <= max_i256 {
            let bytes = self.to_be_bytes();
            Some(I256::from_be_bytes(&self.env(), &bytes))
        } else {
            None
        }
    }
}

// Implement I256ToU256 for I256
impl I256ToU256 for I256 {
    fn to_u256(&self) -> Option<U256> {
        // Convert to I256 only if the U256 is positive i.e >= 0 
        let zero = I256::from_i32(&self.env(), 0);
        if *self >= zero {
            let bytes = self.to_be_bytes();
            Some(U256::from_be_bytes(&self.env(), &bytes))
        } else {
            None
        }
    }

    fn abs_u256(&self) -> U256 {
        let zero = I256::from_i32(&self.env(), 0);
        let min_i256 = I256::from_parts(&self.env(), -0x8000000000000000, 0, 0, 0);

        if *self == min_i256 {
            // Handle the minimum I256 explicitly
            return U256::from_parts(&self.env(), 0x8000000000000000, 0, 0, 0);
        }

        if *self >= zero {
            // Positive numbers: direct conversion to U256
            self.to_u256().unwrap()
        } else {
            // Negative numbers: negate and convert to U256
            let negated = zero.sub(self);
            negated.to_u256().unwrap()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Bytes, Env, I256, U256};

    #[test]
    fn test_u256_to_i256() {
        let env = Env::default();

        // Test with a small positive number
        let small_positive = U256::from_u32(&env, 42);
        assert_eq!(small_positive.to_i256().unwrap(), I256::from_i32(&env, 42));

        // Test with zero
        let zero = U256::from_u32(&env, 0);
        assert_eq!(zero.to_i256().unwrap(), I256::from_i32(&env, 0));

        // Test with maximum I256 value
        let max_i256 = U256::from_parts(&env, 0x7fffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff);
        assert!(max_i256.to_i256().is_some());

        // Test with a value just above maximum I256
        let above_max_i256 = max_i256.add(&U256::from_u32(&env, 1));
        assert!(above_max_i256.to_i256().is_none());

        // Test with maximum U256 value
        let max_u256 = U256::from_parts(&env, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff);
        assert!(max_u256.to_i256().is_none());
    }

    #[test]
    fn test_i256_to_u256() {
        let env = Env::default();

        // Test with a small positive number
        let small_positive = I256::from_i32(&env, 42);
        assert_eq!(small_positive.to_u256().unwrap(), U256::from_u32(&env, 42));

        // Test with zero
        let zero = I256::from_i32(&env, 0);
        assert_eq!(zero.to_u256().unwrap(), U256::from_u32(&env, 0));

        // Test with maximum I256 value
        let max_i256 = I256::from_parts(&env, 0x7fffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff);
        assert!(max_i256.to_u256().is_some());

        // Test with a small negative number
        let small_negative = I256::from_i32(&env, -1);
        assert!(small_negative.to_u256().is_none());

        // Test with minimum I256 value
        let min_i256 = I256::from_parts(&env, -0x8000000000000000, 0, 0, 0);
        assert!(min_i256.to_u256().is_none());
    }

    #[test]
    fn test_i256_abs_u256() {
        let env = Env::default();

        // Test with a small positive number
        let small_positive = I256::from_i32(&env, 42);
        assert_eq!(small_positive.abs_u256(), U256::from_u32(&env, 42));

        // Test with zero
        let zero = I256::from_i32(&env, 0);
        assert_eq!(zero.abs_u256(), U256::from_u32(&env, 0));

        // Test with a small negative number
        let small_negative = I256::from_i32(&env, -42);
        assert_eq!(small_negative.abs_u256(), U256::from_u32(&env, 42));

        // Test with maximum I256 value
        let max_i256 = I256::from_parts(&env, 0x7fffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff);
        assert_eq!(max_i256.abs_u256(), U256::from_parts(&env, 0x7fffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff));

        // Test with minimum I256 value using byte representation
        let min_i256_bytes = [
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ];
        let min_i256_bytes_converted = Bytes::from_array(&env, &min_i256_bytes);
        let min_i256 = I256::from_be_bytes(&env, &min_i256_bytes_converted);
        assert_eq!(min_i256.abs_u256(), U256::from_parts(&env, 0x8000000000000000, 0, 0, 0));

        // Test with a value just greater than the minimum I256 (-0x7FFFFFFFFFFFFFFFF...)
        let slightly_greater_min_bytes = [
            0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        ];
        let slightly_greater_min_bytes_converted = Bytes::from_array(&env, &slightly_greater_min_bytes);
        let slightly_greater_min = I256::from_be_bytes(&env, &slightly_greater_min_bytes_converted);
        assert_eq!(slightly_greater_min.abs_u256(), U256::from_parts(&env, 0x7fffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff));
    }
}