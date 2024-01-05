#[allow(warnings)]

#[derive(Debug)]
struct i64MulResult {
    result: i64,
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
            result: product,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_numbers() {
        let result = i64MulResult::mul_ext(3, 4);
        assert_eq!(result.result, 12);
    }

    #[test]
    fn test_with_zero() {
        let result = i64MulResult::mul_ext(0, 100);
        assert_eq!(result.result, 0);
    }

    #[test]
    fn test_negative_numbers() {
        let result = i64MulResult::mul_ext(-5, 6);
        assert_eq!(result.result, -30);
    }

    #[test]
    fn test_large_numbers() {
        let result = i64MulResult::mul_ext(i32::MAX, 2);
        assert_eq!(result.result, 4294967294);
    }

    #[test]
    fn test_mixed_signs() {
        let result = i64MulResult::mul_ext(-123456, 987654);
        assert_eq!(result.result, -121931812224);
    }
}
