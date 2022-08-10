
/**
函数项默认实现了 Copy / Clone / Sync / Send / Fn /FnMut / FnOnce
 */
#![allow(unused)]
fn main() {
    println!("Hello, world!");
    test1();
}

fn test1() {
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    let s1 = sum;
    let x = s1(1, 2);
    println!("x={}", x);
}