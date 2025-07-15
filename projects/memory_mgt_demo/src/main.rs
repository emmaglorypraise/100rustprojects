use std::rc::Rc;
use std::cell::RefCell;
 
fn main() {
    println!("🧠 Memory Management Demo in Rust");
 
    // Ownership
    let s1 = String::from("Ownership Example");
    let s2 = s1; // s1 moved
    println!("🔑 Ownership transferred: {}", s2);
    // println!("{}", s1); // ❌ Error: s1 moved
 
    // Borrowing
    let s3 = String::from("Borrowing Example");
    borrow_demo(&s3);
    println!("✅ After borrow: {}", s3); // still accessible
 
    // Mutable Borrowing
    let mut s4 = String::from("Hello");
    mutate_demo(&mut s4);
    println!("🔧 After mutation: {}", s4);
 
    // Lifetimes
    let result;
    let a = String::from("abcd");
    {
        let b = String::from("xyz");
        result = longest(&a, &b);
        println!("⏳ Longest string: {}", result);
    }
 
    // Box (heap allocation)
    let boxed = Box::new(42);
    println!("📦 Boxed value: {}", boxed);
 
    // Rc (reference-counted pointer)
    let rc_val = Rc::new(String::from("Shared"));
    let rc_clone = Rc::clone(&rc_val);
    println!("📚 Rc values: {}, {}", rc_val, rc_clone);
    println!("Ref count: {}", Rc::strong_count(&rc_val));
 
    // RefCell (interior mutability)
    let cell = RefCell::new(100);
    *cell.borrow_mut() += 50;
    println!("🧪 RefCell value: {}", cell.borrow());
}
 
fn borrow_demo(data: &String) {
    println!("📥 Borrowed: {}", data);
}
 
fn mutate_demo(data: &mut String) {
    data.push_str(" World");
}
 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
