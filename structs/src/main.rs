// Rust 提供三种结构体：
//
// Named-Field Struct
// Tuple-Like Struct
// Unit-Like Struct

use std::fmt;
use std::ops::Deref;

fn main() {
    println!("\n//1.定义结构体");
    test1();

    println!("\n//2.创建结构体实例");
    test2();

    println!("\n//3.修改结构体字段");
    test3();

    println!("\n//4.参数名字和字段名字同名的简写方法");
    test4();

    println!("\n//5.从其它结构体创建实例");
    test5();

    println!("\n//6.元组结构体");
    test6();

    println!("\n//7.没有任何字段的类单元结构体");
    test7();

    println!("\n//8.打印结构体");
    test8();

    println!("\n//9.结构体添加成员方法");
    test9();

    println!("\n//10.结构体封装 getter 方法");
    test10();

    println!("\n//11.单元结构体");
    test11();

    println!("\n//12.元组结构体-newtype-impl-Deref-当做原始类型使用");
    test12();

    println!("\n//13.元组结构体-newtype-impl-Deref-当做原始类型使用");
    test13();

    println!("\n//14.verify bug #74739");
    test14();

    println!("\n//15.结构体解构");
    test15();
}

//1.定义结构体
fn test1() {
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }
}

//2.创建结构体实例
fn test2() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }

    // 非 mut 变量是不能修改的
    let user = User {
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };
    println!("{:#?}", user);
}

//3.修改结构体字段
fn test3() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }
    // 这里要定义成 mut 变量，才能修改
    let mut user = User {
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };
    println!("{:#?}", user);
    user.name = "jack".to_string();
    println!("{:#?}", user);
}

//4.参数名字和字段名字同名的简写方法
fn test4() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }
    let id = 101;
    let name = "leo".to_string();
    let is_active = true;
    let user = User {
        id, //这里因为 参数名字和字段名字同名，所以就只用写一次
        name,
        is_active,
    };
    println!("{:#?}", user);
}

//5.从其它结构体创建实例
fn test5() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }

    let leo = User {
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };

    let jack = User {
        name: "jack".to_string(),
        ..leo
    };
    println!("{:#?}", jack);
}

//6.元组结构体
fn test6() {
    //元组结构体
    //1.圆括号代替花括号
    //2.字段没有名字，只有类型
    #[derive(Debug)]
    struct Point(i32, i32);

    let p = Point(201, 197);
    println!("{:#?}", p);
    println!("x={}, y={}", p.0, p.1); //通过元组tuple的方式引用
}

//7.没有任何字段的类单元结构体
fn test7() {
    #[derive(Debug)]
    struct Serializable;

    let s = Serializable;
    println!("{:#?}", s);
}

//8.打印结构体
fn test8() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }

    // 非 mut 变量是不能修改的
    let user = User {
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };
    println!("{:#?}", user);
}

//9.结构体添加成员方法
fn test9() {
    #[derive(Debug)]
    struct Serializable {}

    impl Serializable {
        pub fn print(&self) {
            //成员方法一定要加 &self, 返回本类型用 -> Self
            println!("{self:p}"); //打印变量自身地址
        }
    }

    let s = Serializable {};
    s.print();
}

//10.结构体封装 getter 方法
fn test10() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        is_active: bool,
    }

    impl User {
        pub fn get_id(&self) -> u32 {
            //成员方法一定要加 &self, 返回本类型用 -> Self
            self.id
        }
    }
    // 同一个结构体的多个方法，可以分开多次实现，不像java必须在同一个class里面写完
    impl User {
        pub fn get_name(&self) -> &str {
            //成员方法一定要加 &self, 返回本类型用 -> Self
            self.name.as_ref()
        }
    }

    let user = User {
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };

    println!("{}", &user.get_id());
    println!("{}", &user.get_name());
}

fn test11() {
    // 等价于  struct Empty {}
    struct Empty;

    let x = Empty;
    println!("{:p}", &x);

    let y = x;
    println!("{:p}", &y as *const _);

    let z = Empty;
    println!("{:p}", &z as *const _);

    // struct RangeFull;  // 标准库源码中RangeFull就是一个单元结构体
    assert_eq!((..), std::ops::RangeFull); //  RangeFull就是(..)，表示全范围
}

fn test12() {
    struct Wrapper(Vec<String>);

    // newtype 可以绕过孤儿规刚，帮外部类型实现外部 trait
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    // 因为Wrapper是一个新的类型，所以它没有自己内部值的方法。
    // 为了让Wrapper的行为与Vec完全一致，我们需要在Wrapper中实现所有Vec的方法，并将这些方法委托给self.0。
    // 假如我们希望新类型具有内部类型的所有方法，那么我们也可以为Wrapper实现Deref trait来直接返回内部的类型。
    // 假如我们不希望Wrapper类型具有内部类型的所有方法，比如在需要限制Wrapper类型的行为时，我们就只能手动实现需要的那部分方法了。
    // https://blog.csdn.net/u014358963/article/details/122852334
    impl Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    let x = w.join(", ");
    println!("{}", x);
}

fn test13() {
    struct Wrapper(i32);

    impl Deref for Wrapper {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let a = Wrapper(100);
    let b = Wrapper(100);
    let c = a.deref() + b.deref();
    // https://blog.csdn.net/u014358963/article/details/122852334
    // let c: i32 = a + b; // 这里为什么不行？test12() 中不是可以直接调用内部字段的全部方法了吗？
    println!("c={}", c);
}

// https://github.com/rust-lang/rust/issues/74739
// https://new.qq.com/omn/20200726/20200726A0PBQQ00.html
fn test14() {
    #[repr(C)] //禁自编译器对结构体的字段重排，涉及到字段对齐问题
    struct Foo {
        x: i32,
    }

    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    let y = foo;
    println!("{}", y.x); // -> 42; expected result: 13
}

fn test15() {
    #[derive(Debug)]
    struct Point { x: i32, y: i32 };
    #[derive(Debug)]
    struct Line {start: Point, end: Point};

    // let 在这里的作用是模式匹配，具体是解构与赋值
    let Line{start, end} = Line {
        start: Point{x: 102, y: 215},
        end:   Point{x: 123, y: 197}
    };

    println!(
        "Line start({startX:?}, {startY:?}), end({endX:?}, {endY:?})",
        startX = start.x,
        startY = start.y,
        endX = end.x,
        endY = end.y,
    );
}