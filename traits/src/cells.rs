use std::cell::{Cell, RefCell};

#[cfg(test)]
#[test]
fn test1() {
    println!("\n//1.Cell类型");
    let a = Cell::new("good");
    let b = a.get();
    a.set("luck");
    let c = a.get();
    println!("b={}, c={}", b, c);
}

#[test]
fn test2() {
    let s1 = RefCell::new(String::from("good"));
    let s2 = s1.borrow();
    // let s3 = s1.borrow_mut();
    // println!("s2={}, s3={}", s2, s3);
    // println!("s1={:?}, s2={}, s3={}", s1, s2, s3);
}