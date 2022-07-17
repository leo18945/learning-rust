const MAX_POINT: u32 = 10_1000;

fn main() {
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

    test3();
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