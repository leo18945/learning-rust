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

    // enum 比较
    test7();

    // enum 比较
    test8();
}

// 枚举进化之 - 最原始的写法
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

// 枚举进化之 - 枚举项直接包含值
// 背后的原理就是，编译器帮忙生成了struct来包装每一个枚举项的值
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

fn test7() {
    #[derive(Debug, PartialEq)]
    enum IPAddr {
        V4(String),
        V6(String),
    }

    let ip1 = IPAddr::V4(String::from("127.0.0.1"));
    let ip2 = IPAddr::V4(String::from("127.0.0.1"));
    let res = ip1 == ip2;
    println!("ip1 =? ip2 -> {}", res);
}

fn test8() {
    #[derive(PartialEq)]
    enum Attribute {
        Strength,
        Agility,
        Intellect,
    }

    #[derive(PartialEq)]
    enum Parameter {
        Health,
        Mana,
    }

    #[derive(PartialEq)]
    enum BuffTarget {
        Attribute(Attribute),
        Parameter(Parameter),
    }

    let type_1 = BuffTarget::Attribute(Attribute::Strength);
    let type_2 = BuffTarget::Attribute(Attribute::Intellect);

    assert_eq!((type_1 == type_2), false);
}


//---------------------------------------//
/***
1.枚举原理
// https://www.zhihu.com/question/452956370/answer/1819873974
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}
上面的枚举会被翻译成类似这样
struct _Message {
    tag: int,
    values: union {
        struct Quit {}

        struct Move {
            int32_t x, y;
        }

        struct ChangeColor {
            int32_t _1, _2, _3;
        }
    }
}

------------------------------------------
https://www.zhihu.com/question/450752789/answer/1798867075
enum Message {
    Toggled(bool),
}

// 编译器自动生成如下:
impl Message {
    fn Toggled(_arg: bool) -> Self {
        Message::Toggled(_arg)
    }
}

------------------------------------------
https://aloso.github.io/2021/03/10/rusts-universes.html

struct Foo(Bar);

// the compiler transforms the above into something like this:

struct Foo { 0: Bar };

fn Foo(_0: Bar) -> Foo {
    Foo { 0: _0 }
}

*/
//---------------------------------------//

// struct S {
//     a: T1,
//     u: union { b: T2, c: T3 }
// }

