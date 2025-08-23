use testing_suite::{add, divide};
 
#[test]
fn test_add_integration() {
    assert_eq!(add(100, 200), 300);
}
 
#[test]
fn test_divide_integration() {
    let result = divide(9.0, 3.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3.0);
}