/**
闭包
*/
fn main() {
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

    let closures_example = |x|x;
    let s1 = closures_example(String::from("abc123"));
    println!("{}", s1);

    // let s2 = closures_example(1984); //不能推导两次，因为前面已推导成 String 类型了
    // println!("{}", s2);
}


