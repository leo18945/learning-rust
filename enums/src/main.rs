fn main() {
    // 1.C 语言的方式定义
    test1();

    // 2.Rust 提倡的方式定义
    test2();

    //3.Enum 中的元素可以是不同类型
    test3();

    //4.经典用法
    test4();

    //5.枚举类型的方法以及 match
    test5();

    // if let 写法
    test6();
}

fn test1() {
    #[derive(Debug)]
    enum IPAddrKind {
        V4, V6
    }

    #[derive(Debug)]
    struct IPAddr {
        kind: IPAddrKind,
        addr: String,
    }

    let ip1 = IPAddr{
        kind: IPAddrKind::V4,
        addr: String::from("127.0.0.1")
    };

    println!("{:?}", ip1);
}

fn test2() {
    #[derive(Debug)]
    enum IPAddr {
        V4(String),
        V6(String),
    }

    let ip1 = IPAddr::V4(String::from("127.0.0.1"));
    println!("{:?}", ip1);
}

fn test3() {
    #[derive(Debug)]
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let ip1 = IPAddr::V4(192, 168, 1, 114);
    println!("{:?}", ip1);
}

fn test4() {
    let num1 = 1;
    let num2: Option<i32> = Some(1);
    let sum = num1 + num2.unwrap();
    let sum2 = match num2 {
        Some(val) => val,
        None => panic!("called `Option::unwrap()` on a `None` value")
    };
    println!("sum={}", sum);
    println!("sum2={}", sum2);
}

fn test5() {
    fn plus_one(v: Option<i32>) -> Option<i32> {
        match v {
            None => None,
            Some(val) => Some(val + 1)
        }
    }

    let res = plus_one(Some(2));
    println!("res={}", res.unwrap());

    // let res = plus_one(None);
    // println!("res={}", res.unwrap());
}

fn test6() {
    fn plus_one(v: Option<i32>) -> Option<i32> {
        match v {
            None => None,
            Some(val) => Some(val + 1)
        }
    }

    if let Some(res) = plus_one(Some(2)) {
        println!("res={}", res);
    }

    if let Some(res) = plus_one(None) {
        println!("res={}", res);
    } else {
        println!("None")
    }
}