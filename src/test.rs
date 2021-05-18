#[test]
pub fn test_method() {
    assert_eq!(1, 1);
}
 
#[test]
fn test_method2() {
    assert_eq!(1, 3);
}

pub fn other_method() {
    println!("method in another file called");
}