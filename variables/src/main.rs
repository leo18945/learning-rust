use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::usize;
use rust_decimal_macros::dec;

const MAX_POINT: u32 = 10_1000;

fn main() {
    println!("\n//1.test1==================");
    test1();

    println!("\n//3.test3==================");
    test3();

    println!("\n//4.test4==================");
    test4();

    println!("\n//5.test5==================");
    test5();

    println!("\n//6.test6==================");
    test6();
}
fn test1 () {
    println!("Hello, world!");

    let a = 1;

    // a = 2;
    let a = 2;
    println!("{a}");

    const MAX_POINT: u32 = 20_1000;
    println!("{MAX_POINT}");

    let b: i32 = "42".parse().expect("Not a number");
    println!("{b}");

    let c = 57u8;
    println!("{c}");

    let d = 0b1001_1001;
    println!("{d}");

    let e = 2.1;
    println!("{e}");

    let f = 'f';
    println!("{f}");

    let f = '🐬';
    println!("{f}");

    let g = (1, 2, 3);
    let g: (i32, i32, i32) = (1, 2, 3);
    println!("{}, {}, {}", g.0, g.1, g.2);

    let (t0, t1, t2) = g;
    println!("{}, {}, {}", t0, t1, t2);

    let arr = [0, 1, 2, 3, 4, 5];
    println!("{}", arr[0]);
    let arr2 = &arr[1..3];
    println!("{}", arr2[0]);

    let h = {
        let x = 1;
        x + 2
    };
    println!("{h}");

    let str1 = String::from("abc123dev");
    let str2 = &str1[3..];
    println!("{}", str2);

    // drop(1);
}

fn test3() {
    let x: i32;
    if true {
        x = 1;
    } else {
        x = 2;
    }
    println!("x = {}", x);
}

fn test4() {
    let mut f = std::fs::File::open("/Users/leo18945/CLionProjects/rust/learning-rust/variables/src/out.txt").ok().expect("Couldn’t open foo.txt");
    let buf = b"whatever"; //  buf: &[u8; 8]
    let result = f.write(buf); // 此处要导入 use std::io::Write， 才能使用
    // match result {
    //     usize(v) => println!("v={}", v),
    //     Error(e) => println!("error={}", e),
    // }
    result.err();
    // println!("write size={}", result.err());
}

fn test5() {
    let f1 = 0.1;
    let f2 = 0.2;
    println!("{} + {} = {}", f1, f2, f1 + f2);
    if f1 + f2 == 0.3 {
        println!("0.1 + 0.2 = 0.3");
    } else {
        println!("0.1 + 0.2 != 0.3");
    }
}

fn test6() {
    let f1 = dec!(0.1);
    let f2 = dec!(0.2);
    let f3 = f1 + f2;
    println!("f3={}", f3.to_string());
    assert_eq!(f3.to_string(), "0.3");

    let s = String::default();
    println!("s={s}");

}