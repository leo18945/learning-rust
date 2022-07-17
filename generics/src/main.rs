use std::fmt;
use std::fmt::Display;
use std::io::Take;
use regex::Regex;

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

fn main() {
    println!("Hello, world!");
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

    test5();
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

