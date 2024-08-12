<!-- 
impl Logarithm for SoroNum<u128> {
    fn log2(&self) -> Option<u32> {
        Some(128 - self.value.leading_zeros() - 1)
    }

    fn log10(&self) -> Option<u32> {
        let mut count = 0;
        let mut num = self.value;
        while num >= 10 {
            num /= 10;
            count += 1;
        }
        Some(count)
    }
}

impl Logarithm for SoroNum<U256> {
    fn log2(&self) -> Option<u32> {
        let bytes = self.value.to_be_bytes();
        let mut non_zero_byte = 0u8;
        let mut non_zero_index = 0usize;
        for (i, byte) in bytes.iter().enumerate() {
            if byte != 0 {
                non_zero_byte = byte;
                non_zero_index = i;
                break;
            }
        }

        // If the entire U256 is zero, return None
        if non_zero_byte == 0 {
            return None;
        }

        // Calculate log2 based on the position of the first non-zero byte
        let bits_from_most_significant_byte = 255 - (non_zero_index as u32 * 8);
        let leading_zeros_in_byte = non_zero_byte.leading_zeros();
        Some(bits_from_most_significant_byte - leading_zeros_in_byte)
    }

    fn log10(&self) -> Option<u32> {
        // Direct calculation or iterative division by 10 isn't feasible for U256,
        // so we'll use an approximation based on significant digits and `log2`.
        self.log2().map(|log2_val| {
            // Since log2(10) is approximately 3.32193, we use a ratio of 10:3 for approximation.
            // This is a simplification for integer arithmetic, acknowledging potential rounding errors.
            log2_val / 3
        })
    }
}

impl Logarithm for SoroNum<I256> {
    fn log2(&self) -> Option<u32> {
        let bytes = self.value.to_be_bytes();
        let mut non_zero_byte = 0u8;
        let mut non_zero_index = 0usize;
        for (i, byte) in bytes.iter().enumerate() {
            if byte != 0 {
                non_zero_byte = byte;
                non_zero_index = i;
                break;
            }
        }

        // If the entire I256 is zero, or negative, return None
        if non_zero_byte == 0 {
            return None;
        }

        // Calculate log2 based on the position of the first non-zero byte
        let bits_from_most_significant_byte = 255 - (non_zero_index as u32 * 8);
        let leading_zeros_in_byte = non_zero_byte.leading_zeros();
        Some(bits_from_most_significant_byte - leading_zeros_in_byte)
    }

    fn log10(&self) -> Option<u32> {
        // Similar approach as U256, ensuring we only apply this to non-negative values.
        if self.value.to_i128().map_or(true, |v| v <= 0) {
            None
        } else {
            self.log2().map(|log2_val| {
                // Using the same approximation ratio as for U256.
                log2_val / 3
            })
        }
    }
} -->