// 属性宏语句
// 意为允许未使用的变量或函数
#![allow(unused)]

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

    println!("\n//7.test7==================");
    test7();

    println!("\n//8.test8==================");
    test8();

    println!("\n//9.test9==================");
    test9();

    println!("\n//10.test10==================");
    test10();

    println!("\n//11.test11==================");
    test11();

    println!("\n//12.test12==================");
    test12();

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

    // 可以使用非 ascii 字符做为变量名
    let 品牌 = "宝马".to_string();
    println!("{}", 品牌);

    let _品牌 = "大众".to_string();
    println!("{}", _品牌);

    // let _😀😀😀👨‍👩‍👦‍👦🧶🦅_ = "好复杂的变量名".to_string();
    // println!("{}", _😀😀😀👨‍👩‍👦‍👦🧶🦅_);

}

fn test7() {
    const pi: f32 = gen_pi();
    let x = pi * 2.10;
    println!("x={}", x);
    println!("&pi={:p}", &pi);
    println!("&x={:p}", &x);

    let y = gen_pi();
    let z = y * 3.12;
    println!("z={}", z);

    // cannot call non-const fn `gen_pi3` in constants
    // calls in constants are limited to constant functions, tuple structs and tuple variants
    // const pi3: f32 = gen_pi3();
    // println!("pi3={}", pi3);

    let a1 = (101, ).0;
    // `{integer}` is a primitive type and therefore doesn't have fields
    // let a1 = (101).0;

    println!("gcd(21, 7)={}", GCD);

    println!("fib(10)={}", FIBS);
}

fn test8() {
    // use of possibly-uninitialized `a`
    // 这是因为会把 while 后面当一个表达式，这个表达式是true还是false，不是在编译期确定的，而是在运行时确定的，
    // 所以在编译的时候即然确定不了，那就是说 while 后面的语句不一定会执行
    // 那 while 后面的语句即然不一定会执行，那变量 a 就有可能没有初始化
    // rust 编译器不会让这一切发生，所以就报错了
    // 可以改为 loop {}
    // let mut a;
    // while true {
    //     a = 1;
    //     break;
    // }
    // println!("{}", a);
}

fn test9() {
    // if true 同样不支持
    // let mut a;
    // use of possibly-uninitialized `a`
    // if true {
    //     a = 1;
    // }
    // println!("{}", a);
    // new 也是 const function
    // let s = std::vec::Vec::new();
}

// 不像 let 绑定那样，您必须标注常量的类型。
//
// 常量活在程序的整个生命周期。更具体地说，在 Rust 语言里面常量没有固定内存地址。
// 这是因为他们会被有效的内联到每个使用地方。引用相同的常数并不一定保证引用同一个内存地址也是因为这个原因。
const ppi: f32 = gen_pi() * gen_pi();

const fn gen_pi() -> f32 {
    // 不能在这里打印
    // println!("get pi value");
    3.1415926
}

fn gen_pi3() -> f32 {
    3.1415926
}

// 求最大公约数
const fn gcd(a: u32, b: u32) -> u32 {
    match (a, b) {
        (x, 0) | (0, x) => x,
        (x, y) if x % 2 == 0 && y & 2 == 0 => 2 * gcd(x / 2, y / 2),
        (x, y) | (y, x) if x % 2 == 0 => gcd(x /2, y),
        (x, y) if x < y => gcd((y - x) / 2, x),
        (x, y) => gcd((x - y) / 2, y),
    }
}

const GCD: u32 = gcd(21, 7);

// 求斐波那契数
const fn fib(n: u128) -> u128 {
    const fn helper(n: u128, a: u128, b: u128, i: u128) -> u128 {
        if i <= n {
            helper(n, b, a + b, i + 1)
        } else {
            b
        }
    }
    helper(n, 1, 1, 2)
}

const FIBS: u128 = fib(10);

fn test10() {
    let mut a = 10;
    let     b = &mut a;
    *b = 20;
    // println!("{:?}, {:?}", a, b);
    // println!("{:?}", a);
    println!("{:?}", b);
    // println!("{:?}", a);
    a = 30;
    println!("{:?}", a);
    // println!("{:?}", b);
}

fn test11() {
    let a: bool = true;
    let b: i32 = a as i32; // bool 类型转换为整型
    println!("b={}", b);

    let c: bool = false;
    let d: i32 = c as i32;
    println!("b={}", d);

    // cannot cast as `bool`
    // let e: bool = d as bool;
}

fn test12() {
    fn print_type_of<T>(_: &T) {
        println!("Type is: {}", std::any::type_name::<T>())
    }

    print_type_of(&"Hi!");

    print_type_of(&String::new());

    let mut y = 5;
    let x = (y = 6);
    println!("{:?}", x);
}