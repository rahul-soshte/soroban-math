#[allow(warnings)]

#[derive(Debug)]
struct i64MulResult {
    lower: i32,
    upper: i32,
}

// Define a trait for multiplication that returns an i64MulResult
trait MulExt {
    fn mul_ext(a: i32, b: i32) -> i64MulResult;
}

// Implement the trait for the i64MulResult struct
#[allow(dead_code)]
impl MulExt for i64MulResult {
    fn mul_ext(a: i32, b: i32) -> i64MulResult {
        let product = (a as i64) * (b as i64);
        i64MulResult {
            lower: product as i32,
            upper: (product >> 32) as i32,
        }
    }
}

#[allow(dead_code)]
fn test_combine_to_i64(high: i32, low: i32) -> i64 {
    // Cast the high part to i64 and shift left by 32 bits, then combine with the low part
    ((high as i64) << 32) | ((low as i64) & 0xFFFFFFFF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_numbers() {
        let result = i64MulResult::mul_ext(3, 4);
        assert_eq!(result.lower, 12);
        assert_eq!(result.upper, 0);
    }

    #[test]
    fn test_with_zero() {
        let result = i64MulResult::mul_ext(0, 100);
        assert_eq!(result.lower, 0);
        assert_eq!(result.upper, 0);
    }

    #[test]
    fn test_negative_numbers() {
        let result = i64MulResult::mul_ext(-5, 6);
        assert_eq!(result.lower, -30);
        assert_eq!(result.upper, -1); // Upper part for negative numbers
    }

    #[test]
    fn test_large_numbers() {
        let result = i64MulResult::mul_ext(i32::MAX, 2);

        // Calculate the expected combined i64 result
        let expected_combined_result = i32::MAX as i64 * 2 as i64;

        // Combine the result's upper and lower parts
        let combined_result = test_combine_to_i64(result.upper, result.lower);

        // Compare the combined result with the expected result
        assert_eq!(combined_result, expected_combined_result);
    }

    #[test]
    fn test_mixed_signs() {
        let result = i64MulResult::mul_ext(-123456, 987654);
        // Calculate the expected upper and lower parts of the result
        let expected_lower = -121931812224i64 as i32;
        let expected_upper = (-121931812224i64 >> 32) as i32;
        assert_eq!(result.lower, expected_lower);
        assert_eq!(result.upper, expected_upper);
    }
}
