use std::fmt;
use std::fmt::Display;
use std::io::Take;
use regex::Regex;
use std::ops::Add;

// Rust 中的泛型
// https://juejin.cn/post/6909067849868771342

//参考 traits/src/main.rs
fn main() {
    test1();

    test5();

    // println!("\n// 6.默认泛型类型参灵长与运算符重载");
    println!("\n// 6.重载+号运算符");
    test6();

    println!("\n// 8.重载+号运算符, 重载时指定另一种类型，可用于两种不同类型相加");
    test8();

    println!("\n// 9.重载+号运算符, 重载时指定另一种类型，可用于两种不同类型相加");
    test9();

    println!("\n// 10.泛型手动指定类型");
    test10();
}

#[derive(PartialEq, Eq, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(PartialEq, Eq, Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// impl Display for Point<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "x={}, y={}", self.x, self.y);
//     }
// }

impl <T, U> Point2<T, U> {
    fn get_x(&self) -> &T {
        &(self.x)
    }

    fn get_y(&self) -> &U {
        &(self.y)
    }

    fn create_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

fn test1() {
    let arr = [1, 2, 3, 4, 5];
    let maxv = max_i32(&arr);
    println!("{maxv}");

    let maxv = max_int(&arr);
    println!("{maxv}");

    let arr = ['a', 'b', 'c', 'd', 'e'];
    let maxv = max_int(&arr);
    println!("{maxv}");

    let p = Point{x: 1, y: 2};
    println!("{:?}",  p);
    println!("{:#?}", p);

    let p = Point2{x: 1, y: 2};
    println!("{:?}",  p);
    println!("{:#?}", p);
    println!("{}", p.get_x());

    let p2 = Point2{x: "hello", y: 'c'};
    let p3 = p.create_point(p2);
    println!("{}", p3.x);
}

fn max_i32(list: &[i32]) -> i32 {
    let mut maxv = list[0];
    for &v in list.iter() {
        if v > maxv {
            maxv = v;
        }
    }
    maxv
}

fn max_i64(list: &[i64]) -> i64 {
    let mut maxv = list[0];
    for &v in list.iter() {
        if v > maxv {
            maxv = v;
        }
    }
    maxv
}

fn max_int<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut maxv = list[0];
    for &v in list.iter() {
        if v > maxv {
            maxv = v;
        }
    }
    maxv
}

trait Parse{
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures.get(0).map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else { 0 }
    }
}

fn test5() {
    assert_eq!(u8::parse("123c"), 123);
    assert_eq!(u8::parse("c123"), 0);
}

//默认泛型类型参数和运算符重载
//  1)使用泛型类型参数时，可以为泛型指定一个默认的具体的类型
//  2)运算符重载是指在特定情况下自定义运算符行为的操作
//  Rust 并不允许创建自定义运算符或者重载运算符，不过地于 std::ops中列出的运算符和相应的 trait, 我们可以实现运算符相关 trait 来重载

// 重载+号运算符
fn test6() {
    #[derive(Debug, PartialEq)]
    struct Point{
        x : i32,
        y : i32,
    }

    // pub trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Self) -> Self::Output {
            Point {
                x : self.x + other.x,
                y : self.y + other.y,
            }
        }
    }

    let p1 = Point {x : 1, y : 3};
    let p2 = Point {x : 4, y : 7};
    assert_eq!(p1 + p2, Point {x : 5, y : 10});
}

fn test7() {
    #[derive(Debug, PartialEq)]
    struct Point{
        x : i32,
        y : i32,
    }

    // Rhs 为泛型，= Self 为默认值，也就是说如果在实现Add trait时不写类型时，就用Self这个类型
    pub trait Add<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Self) -> Self::Output {
            Point {
                x : self.x + other.x,
                y : self.y + other.y,
            }
        }
    }

    // let p1 = Point {x : 1, y : 3};
    // let p2 = Point {x : 4, y : 7};
    // assert_eq!(p1 + p2, Point {x : 5, y : 10});
}

fn test8() {
    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    impl Add<Millimeters> for Meters {
        type Output = Millimeters;

        fn add(self, rhs: Millimeters) -> Self::Output {
            Millimeters(self.0 * 1000 + rhs.0)
        }
    }

    let m1 = Millimeters(10);
    let m2 = Meters(1);
    assert_eq!(m1 + m2, Millimeters(1010));

    let m1 = Millimeters(10);
    let m2 = Meters(1);
    // cannot add `Millimeters` to `Meters`
    // an implementation of `std::ops::Add<_>` might be missing for `Meters`
    // m2 + 其实相当于 m2.add，所以说左边的类型如果没有实现 Add trait 的话，是不能执行+运算的
    assert_eq!(m2 + m1, Millimeters(1010));
}

fn test9() {
    # [derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Self) -> Self::Output {
            Millimeters(self.0 + rhs.0)
        }
    }

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    let m1 = Millimeters(10);
    let m2 = Millimeters(5);

    let m3 = Millimeters(10);
    let m4 = Meters(1);

    let res1 = m1 + m2;
    let res2 = m3 + m4;

    println!("res1={:?}", res1);
    println!("res2={:?}", res2);
}

fn test10() {
    fn foo<T>(t: T) -> T {
        t
    }
    assert_eq!(foo(3), 3);
    assert_eq!(foo("hello"), "hello");
    // 上面两行调用与下面两行调和等价
    assert_eq!(foo::<i32>(3), 3); // turbofish 操作符
    assert_eq!(foo::<&'static str>("hello"), "hello");
}