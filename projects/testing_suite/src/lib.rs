pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
 
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(a / b)
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
 
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }
 
    #[test]
    fn test_divide_by_zero() {
        assert!(divide(5.0, 0.0).is_err());
    }
}
