/**
1.闭包是可以保存进变量或者作为参数传递给其它函数的匿名函数。闭包和函灵长不同的是，闭包允许捕获调用者作用域中的值
2.闭包的使用方式
3.使用带泛型和Fn trait的闭包
*/

/**
函数说明：
1.函数都拥有显式的类型签名
2.函数可以分为三种类型：自由函数，关联函数，方法
3.函数自身也是一种类型
*/

fn main() {
    println!("\n1.闭包的基本使用方式");
    test1();

    println!("\n2.闭包的基本使用方式");
    test2();
}

fn test1() {
    fn  add_one_v1               (x: i32) -> i32 { x + 1 }
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    // 编译器会为闭包中的参数与返回值推导类型，但是不能推导两次
    let add_one_v3    = |x|             { x + 1 };
    let add_one_v4    = |x|               x + 1  ;

    let a = add_one_v1(1);
    println!("{a}");

    let a = add_one_v2(2);
    println!("{a}");

    let a = add_one_v3(3);
    println!("{a}");

    let a = add_one_v4(4);
    println!("{a}");

    let closures_example = | x: String | format!("****{}****", x);
    let s1 = closures_example(String::from("abc123"));
    println!("{}", s1);

    // let s2 = closures_example(1984); //不能推导两次，因为前面已推导成 String 类型了
    // println!("{}", s2);
}

fn test2() {

}


