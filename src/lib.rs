#[allow(warnings)]

#[derive(Debug)]
struct u64MulResult {
    lower: u32,
    upper: u32,
}

// Define a trait for multiplication that returns a u64MulResult
trait MulExt {
    fn mul_ext(a: u32, b: u32) -> u64MulResult;
}

// Implement the trait for the u64MulResult struct
#[allow(dead_code)]
impl MulExt for u64MulResult {
    fn mul_ext(a: u32, b: u32) -> u64MulResult {
        let product = (a as u64) * (b as u64);
        u64MulResult {
            lower: product as u32,
            upper: (product >> 32) as u32,
        }
    }
}

#[allow(dead_code)]
fn test_combine_to_u64(high: u32, low: u32) -> u64 {
    ((high as u64) << 32) | (low as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_numbers() {
        let result = u64MulResult::mul_ext(3, 4);
        assert_eq!(result.lower, 12);
        assert_eq!(result.upper, 0);
    }

    #[test]
    fn test_with_zero() {
        let result = u64MulResult::mul_ext(0, 100);
        assert_eq!(result.lower, 0);
        assert_eq!(result.upper, 0);
    }

    #[test]
    fn test_max_values() {
        let result = u64MulResult::mul_ext(u32::MAX, u32::MAX);
        assert_eq!(result.lower, 1);
        assert_eq!(result.upper, 4294967294);
    }

    #[test]
    fn test_one_operand_max() {
        let result = u64MulResult::mul_ext(u32::MAX, 2);
        assert_eq!(result.lower, 4294967294);
        assert_eq!(result.upper, 1);
    }

    #[test]
    fn test_large_numbers() {
        let result = u64MulResult::mul_ext(123456789, 987654321);
        assert_eq!(test_combine_to_u64(result.upper, result.lower), 121932631112635269);
    }
}
