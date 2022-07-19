pub mod leotest;

use std::fmt::{Debug, Display, Formatter};
use std::mem::size_of_val;
use crate::leotest::practise_trait_1::model1::{To_String, User};

//参考 generics/src/main.rs
fn main() {
    println!("\n//1.trait用于定义与其它类型共享的功能，类似于其它语言中的接口");
    println!("\n//  1)可以通过 trait 以抽象的方式定义共享的行为");
    println!("\n//  2)可以使用 trait bounds 指定泛型是任何拥有特定行为的类型");
    test1();

    println!("\n//2.定义 trait");
    test2();

    println!("\n//3.实现 trait");
    test3();

    println!("\n//4.默认实现：可以在定义 trait 时提供默认行为，trait的类型可以使用默认的行为");
    test4();

    println!("\n//5.trait 作为参数");
    test5();

    println!("\n//6.函数返回值用 trait 限定");
    test6();

    println!("\n//7.函数返回值用 trait 限定，返回不确定的类型");
    test7();

    println!("\n//8.函数泛型用 trait 限定");
    test8();

    println!("\n//9.使用 trait bound 有条件的实现方法");
    test9();

    println!("\n//10.有条件的实现 trait法");
    test10();

    println!("\n//11.用 [泛型]-实现限定 trait 类型");
    test11();

    println!("\n//12.用 [关联类型]-实现限定 trait 类型");
    test12();

    println!("\n//13.全限定语法");
    test13();

    println!("\n//14.全限定语法");
    test14();

    println!("\n//15.全限定语法");
    test15();

    println!("\n//16.全限定语法");
    test16();

    println!("\n//17.类型别名");
    test17();

    println!("\n//18.测试 PartialEq，查看一些数据类型的比较方式，是按值还是按引用比较？");
    test18();

    println!("\n//19.TryInto Trait");
    test19();

    println!("\n//20.From ~ Into 实例");
    test20();

    println!("\n//21.From ~ Into 精简代码实例");
    test21();

    println!("\n//22.Drop Trait");
    test22();
}

fn test1() {
    pub trait GetInfo {
        fn get_name(&self) -> &String;
        fn get_age(&self) -> u32;
    }

    #[derive(Debug)]
    pub struct Student {
        name: String,
        age: u32,
    }

    impl GetInfo for Student {
        fn get_name(&self) -> &String {
            &self.name
        }

        fn get_age(&self) -> u32 {
            self.age
        }
    }

    let s = Student { name: "leo".to_string(), age: 28 };
    println!("s->{:#?}", s);
    println!("s.name->{}", s.get_name());
    println!("s.age->{}", s.get_age());
}

fn test2() {
    let leo = User { id: 101, name: "leo".to_string() };
    println!("{}", leo.to_string());
}

fn test3() {
    // 1.类型约束的第一种写法
    // 要求要打印的对象，必须实现了 To_String trait，才能打印
    fn print_info(item: impl To_String) {
        println!("{}", item.to_string());
    }
    let leo = User { id: 101, name: "leo".to_string() };
    print_info(leo);

    // 2.类型约束的第二种写法: trait bound 写法
    fn print_info_2<T: To_String>(item: T) {
        println!("{}", item.to_string());
    }
    let leo = User { id: 101, name: "leo".to_string() };
    print_info_2(leo);

    // 3.类型约束的第三种写法: where 写法
    fn print_info_3<T>(item: T)
        where T: To_String
    // where T: To_String + Copy
    {
        println!("{}", item.to_string());
    }
    let leo = User { id: 101, name: "leo".to_string() };
    print_info_3(leo);
}

//4.默认实现：可以在定义 trait 时提供默认行为，trait的类型可以使用默认的行为
fn test4() {
    struct User {
        id: u32,
        name: String,
    }

    trait Version {
        // 默认实现
        fn version(&self) -> String {
            "1.0".to_string()
        }
    }

    impl Version for User {}

    let jacky = User {
        id: 101,
        name: "jacky".to_string(),
    };

    println!("User version->{}", jacky.version());
}

fn test5() {
    println!("test51===================");
    test51();

    println!("test52===================");
    test52();
}

fn test51() {
    trait Exchange {
        fn exchange(&self) {
            println!("good luck");
        }
    }

    impl Exchange for i32 {}

    let a = 10;
    a.exchange();
}

fn test52() {
    // trait Exchange<T> {
    //     fn exchange(&mut self, other: &mut T) {
    //         std::mem::swap(self,other);
    //         // let mut temp = *self;
    //         // self = other;
    //         // other = &temp;
    //     }
    // }
    //
    // impl Exchange<i32> for i32 {}
    //
    // let mut a = 10;
    // let mut b = &20;
    // a.exchange(&mut b);
    // println!("a={}", a);
}

fn test6() {
    struct User {
        id: u32,
        name: String,
    }

    trait Version {
        // 默认实现
        fn version(&self) -> String {
            "1.0".to_string()
        }
    }

    impl Version for User {}

    // 函数返回值使用 trait 限定
    fn get_version() -> impl Version {
        let jacky = User {
            id: 101,
            name: "jacky".to_string(),
        };
        jacky
    }

    let v = get_version();
    println!("v.version->{}", v.version());
}

fn test7() {
    #[derive(Debug)]
    struct Student {
        name: String,
        pub age: u32,
    }

    #[derive(Debug)]
    struct Teacher {
        name: String,
        pub age: u32,
    }

    trait GetAge {
        fn get_age(&self) -> u32;
    }

    impl GetAge for Student {
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    impl GetAge for Teacher {
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    fn produce_item_with_age(t: bool) -> impl GetAge {
        if t {
            Student {
                name: "Jacky".to_string(),
                age: 18,
            }
        } else {

            // `if` and `else` have incompatible types
            // expected struct `test7::Student`, found struct `Teacher`
            // 这里不能返回和上面 Teacher 不一致的类型
            // Teacher {
            //     name: "Micheal".to_string(),
            //     age : 28,
            // }

            Student {
                name: "Jacky".to_string(),
                age: 18,
            }
        }
    }

    let age = produce_item_with_age(true).get_age();
    println!("age={}", age);
}

fn test8() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let list = vec![1, 2, 3, 4, 5];
    let largest = largest(&list);
    println!("largest={}", largest);
}

//9.使用 trait bound 有条件的实现方法
fn test9() {
    trait GetName {
        fn get_name(&self) -> &String;
    }

    trait GetAge {
        fn get_age(&self) -> u32;
    }

    struct PeopleMatchInfo<M, S> {
        teacher: M,
        student: S,
    }

    impl<M: GetName + GetAge, S: GetName + GetAge> PeopleMatchInfo<M, S> {
        fn print_all_info(&self) {
            println!("teacher name  = {}", self.teacher.get_name());
            println!("teacher age   = {}", self.teacher.get_age());

            println!("student name = {}", self.student.get_name());
            println!("student age  = {}", self.student.get_age());
        }
    }

    struct Teacher {
        name: String,
        age: u32,
    }

    impl GetName for Teacher {
        fn get_name(&self) -> &String {
            &self.name
        }
    }

    impl GetAge for Teacher {
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    struct Student {
        name: String,
        age: u32,
    }

    impl GetName for Student {
        fn get_name(&self) -> &String {
            &self.name
        }
    }

    impl GetAge for Student {
        fn get_age(&self) -> u32 {
            self.age
        }
    }

    let teacher = Teacher {
        name: "Will".to_string(),
        age: 37,
    };

    let student = Student {
        name: "Tomas".to_string(),
        age: 13,
    };

    let p = PeopleMatchInfo {
        teacher,
        student,
    };

    p.print_all_info();
}

//10.对任何实现特定 trait 的类型，有条件的实现 trait
fn test10() {
    trait GetName {
        fn get_name(&self) -> &String;
    }

    trait PrintName {
        fn print_name(&self);
    }

    // 要求类型T必须实现了 GetName trait
    // 意思是：如果一个类型实现了 GetName，就自动实现了 PrintName trait，也就是可以直接调用 print_name 方法
    impl<T: GetName> PrintName for T {
        fn print_name(&self) {
            println!("name -> {}", self.get_name());
        }
    }

    struct Student {
        name: String,
        age: u32,
    }

    // Student 实现了 GetName trait
    // 就会自动拥有 PrintName trait 的方法如 print_name
    impl GetName for Student {
        fn get_name(&self) -> &String {
            &self.name
        }
    }

    let mia = Student {
        name: "Mia".to_string(),
        age: 28,
    };

    mia.print_name();
}

//父 trait 用于在另一个 trait 中使用某 trait 的功能
//有时我们可能会需要某个 trait 使用另一个 trait 的功能
//在这种情况下，需要能够依赖相关的 trait 也被实现
//这个所需的 trait 是我们实现的 trait 的父(超) trait (super trait)
fn test15() {
    // Output 继承 Display
    trait Output: Display {
        fn out_print(&self) {
            let str = self.to_string();
            println!("{}", str);
        }
    }

    #[derive(Debug)]
    struct Point(i32, i32);

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl Output for Point {
        // fn out_print(&self) {
        //     todo!()
        // }
    }

    let p = Point(100, 103);
    p.out_print();
    // println!("{:?}", p.out_print());
}

// 用泛型限定 trait 中的类型
fn test11() {
    pub trait Iterator1<T> {
        fn next(&mut self) -> Option<T>;
    }

    struct A {
        value: i32,
    }

    impl Iterator1<i32> for A {
        fn next(&mut self) -> Option<i32> {
            println!("in i32");
            if (self.value > 3) {
                self.value += 1;
                Some(self.value)
            } else {
                None
            }
        }
    }

    impl Iterator1<String> for A {
        fn next(&mut self) -> Option<String> {
            println!("in String");
            if (self.value > 3) {
                self.value += 1;
                let mut s = self.value.to_string();
                s.push_str("->String");
                Some(s)
            } else {
                None
            }
        }
    }

    let mut a = A { value: 4 };
    // let res1 = a.next(); // 这种类型不能自省
    let res2: Option<i32> = a.next(); // 这种也可以调用
    println!("res2={}", res2.unwrap());

    let res3: Option<String> = a.next();
    println!("res3={}", res3.unwrap());

    // 调用用泛型实现的trait时，必须要标注具体的类型，具体用-> A as Iterator2<i32> 表示类型转换为 Iterator2<i32>
    let res4 = <A as Iterator1<i32>>::next(&mut a);
    println!("res4={}", res4.unwrap());

    let res5 = <A as Iterator1<String>>::next(&mut a);
    println!("res5={}", res5.unwrap());
}

//11.关联类型
// 关联类型在 trait 定义中指定占位符类型
// 关联类型是一个将类型占位符与 trait 相关联的方式
// trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型
// 如此可以定义一个使用多种类型的 trait
fn test12() {
    //==============以下开始用关联类型实现==================//
    pub trait Iterator2 {
        type Output;
        fn next(&mut self) -> Option<Self::Output>;
    }

    struct A {
        value: i32,
    }

    impl Iterator2 for A {
        type Output = i32;

        fn next(&mut self) -> Option<Self::Output> {
            println!("in i32");
            if self.value > 3 {
                self.value += 1;
                Some(self.value)
            } else {
                None
            }
        }
    }

    let mut a = A { value: 4 };
    let res1 = a.next(); // 这种类型不能自省
    println!("res1={}", res1.unwrap());
}

//完全限定语法
fn test13() {
    trait A {
        fn print(&self);
    }

    trait B {
        fn print(&self);
    }

    struct MyType;

    impl A for MyType {
        fn print(&self) {
            println!("A trait for MyType");
        }
    }

    impl B for MyType {
        fn print(&self) {
            println!("B trait for MyType");
        }
    }

    impl MyType {
        fn print(&self) {
            println!("print is myself method for MyType");
        }
    }

    let t = MyType {};
    t.print();
    MyType::print(&t);

    //=============================//
    let t = MyType {};
    <MyType as A>::print(&t);
    //以下为上面的等价效果
    A::print(&t);

    //=============================//
    let t = MyType {};
    <MyType as B>::print(&t);
    //以下为上面的等价效果
    B::print(&t);
}

fn test14() {
    trait Animal {
        // trait 的静态方法
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        // struct 的静态方法
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("Puppy")
        }
    }

    println!("Dog::baby_name={}", Dog::baby_name());
    // println!("Dog::baby_name={}", Animal::baby_name()); //静态方法不能这样调，只能用全限定语法调，如下
    println!("Dog::baby_name={}", <Dog as Animal>::baby_name());
}

// newtype 模式用以在外部类型上实现外部 trait
// 孤儿规则 (orphan rule) ：只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait
// （1）如果要实现外部定义的 trait 需要先将其导入作用域
// （2）不允许对外部类型实现外部 trait
// （3）可以对外部类型实现自定义的 trait
// （4）可以对自定义类型上实现外部 trait
// 一个绕开这个限制的方法是使用 newtype 模式 (newtype pattern)
fn test16() {
    let v = vec!["hello".to_string(), "world".to_string()];
    // println!("{}", v); // `Vec<String>` doesn't implement `std::fmt::Display`
    // 想直接打印 Vec<String> 类型的值不行，因为 Vec 没有实现 Display，
    // 解决办法：用 newtype 模式，对 Vec<String> 类型包装一层，再对包装类实现 Display 就行啦，如下

    struct Wrapper(Vec<String>);

    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(v);
    println!("{}", w.to_string()); // to_string 方法会调用 Display::fmt 方法，把变量转换成字符串
    println!("{}", w); //这样也可以调用
}

//类型别名
fn test17() {
    type int = i32;
    let a: i32 = 100;
    let b: int = 200;
    let c = a + b;
    println!("c={}", c);
}

fn test18() {
    let s1 = String::from("luck");
    let s2 = String::from("luck");
    if s1 == s2 {
        println!("s1 & s2 are equals!")
    } else {
        println!("s1 & s2 are not equals!")
    }

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1, 2, 3, 4, 5];
    if v1 == v2 {
        println!("v1 & v2 are equals!")
    } else {
        println!("v1 & v2 are not equals!")
    }

    let sl1 = &v1[..];
    let sl2 = &v2[..];
    if sl1 == sl2 {
        println!("sl1 & sl2 are equals!")
    } else {
        println!("sl1 & sl2 are not equals!")
    }
}

fn test19() {
    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

// From ~ Into 实例
// https://zhuanlan.zhihu.com/p/392826855
fn test20() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    ;

    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Self {
            Point { x, y }
        }
    }

    impl From<Point> for (i32, i32) {
        fn from(Point { x, y }: Point) -> Self {
            (x, y)
        }
    }

    impl From<[i32; 2]> for Point {
        fn from([x, y]: [i32; 2]) -> Self {
            Point { x, y }
        }
    }

    impl From<Point> for [i32; 2] {
        fn from(Point { x, y }: Point) -> Self {
            [x, y]
        }
    }

    //=============================//
    // Point 原始构造
    let p1 = Point { x: 102, y: 147 };
    println!("p1={:?}", p1);

    //=============================//
    // 从 (i32, i32) 到 Point，From 实现
    let p1 = Point::from((100, 107));
    println!("p1={:?}", p1);
    // 从 Point 到 (i32, i32)，From 实现
    let t1: (i32, i32) = <(i32, i32)>::from(p1);
    println!("t1={:?}", t1);

    // 从 (i32, i32) 到 Point，Into 实现
    let p1: Point = (100, 107).into();
    println!("p1={:?}", p1);
    // 从 Point 到 (i32, i32)，Into 实现
    let t1: (i32, i32) = p1.into();
    println!("t1={:?}", t1);

    //=============================//
    // 从 [i32; 2] 到 Point，From 实现
    let p1 = Point::from([201, 127]);
    println!("p1={:?}", p1);
    // 从 Point 到 [i32; 2]，From 实现
    let t1: [i32; 2] = <[i32; 2]>::from(p1);
    println!("t1={:?}", t1);

    // 从 [i32; 2] 到 Point，Into 实现
    let p1: Point = [201, 127].into();
    println!("p1={:?}", p1);
    // 从 Point 到 [i32; 2]，Into 实现
    let t1: [i32; 2] = p1.into();
    println!("t1={:?}", t1);
}

// From ~ Into 实例
// https://zhuanlan.zhihu.com/p/392826855
fn test21() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    ;

    impl Point {
        fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }

    impl From<(i32, i32)> for Point {
        fn from((x, y): (i32, i32)) -> Self {
            Point { x, y }
        }
    }

    #[derive(Debug)]
    struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }
    ;

    impl Triangle {
        fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
            Triangle { p1, p2, p3 }
        }
    }

    impl<P: Into<Point>> From<[P; 3]> for Triangle {
        fn from([p1, p2, p3]: [P; 3]) -> Self {
            Triangle { p1: p1.into(), p2: p2.into(), p3: p3.into() }
        }
    }

    // 原始手动构造
    let t = Triangle {
        p1: Point { x: 1, y: 2 },
        p2: Point { x: 3, y: 4 },
        p3: Point { x: 5, y: 6 },
    };
    println!("Triangle={:?}", t);

    // 使用 Point::new
    let t = Triangle {
        p1: Point::new(1, 2),
        p2: Point::new(3, 4),
        p3: Point::new(5, 6),
    };
    println!("Triangle={:?}", t);

    // 使用 From<(i32, i32)> for Point
    let t = Triangle {
        p1: (1, 2).into(),
        p2: (3, 4).into(),
        p3: (5, 6).into(),
    };
    println!("Triangle={:?}", t);

    // 使用 Triangle::new + From<(i32, i32)> for Point
    let t = Triangle::new(
        (1, 2).into(),
        (3, 4).into(),
        (5, 6).into(),
    );
    println!("Triangle={:?}", t);

    // 使用 From<[Into<Point>; 3]> for Triangle
    let t: Triangle = [
        (1, 2),
        (3, 4),
        (5, 6),
    ].into();
    println!("Triangle={:?}", t);
}

// Drop 实现
// https://blog.csdn.net/weixin_44691608/article/details/121319407
fn test22() {
    #[derive(Debug)]
    struct Point(i32, i32);

    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn drop_resource<T: Debug>(item: &mut T) {
        println!("\t=== [release resource] {:?}, free: {} bytes", item, size_of_val(item));
    }

    impl Drop for Point {
        // 以下代码会调用 drop trait
        // /usr/local/Cellar/rust/1.62.0/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:486
        // pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T)
        // https://juejin.cn/post/7062613016831295501#heading-2
        fn drop(&mut self) {
            drop_resource(self);
        }
    }

    impl Drop for Rectangle {
        fn drop(&mut self) {
            drop_resource(self);
        }
    }

    fn test_1() {
        let p1 = Point(101, 127);
        println!("p1={:?}", p1);
        let p1 = Point(101, 127);
        println!("p1={:?}", p1);
    }
    test_1();

    println!("\n//test_2=====================");
    fn test_2() {
        {
            let p1 = Point(195, 206);
            println!("p1={:?}", p1);
        }
        let p1 = Point(147, 226);
        println!("p1={:?}", p1);
    }
    test_2();

    println!("\n//test_3=====================");
    fn test_3() {
        let rect = Rectangle {
            top_left: Point(1, 1),
            bottom_right: Point(2, 2),
        };
        println!("rect = {:?}", rect);

        let box_rect = Box::new(Rectangle {
            top_left: Point(3, 3),
            bottom_right: Point(4, 4),
        });
        println!("box_rect = {:?}", box_rect);
    }
    test_3();

    println!("\n//test_4=====================");
    fn test_4() {
        let p1 = Point(101, 127);
        println!("p1={:?}", p1);
        drop(p1);
        let p1 = Point(101, 127);
        println!("p1={:?}", p1);
    }
    test_4();
}

// Rust源码组织结构
// https://www.cnblogs.com/mengsuenyan/p/13463834.html
