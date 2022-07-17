
//char类型在rust中是32位长度，而不像java中char是两个字节16位长
fn main() {
    println!("\n//1.数组定义==================");
    test1();

    println!("\n//2.元组定义==================");
    test2();

    println!("\n//3.元组模式匹配, 又叫元组拆解==================");
    test3();

    println!("\n//4.元组模式匹配==================");
    test4();

    println!("\n//5.函数返回值==================");
    let res = test5();
    println!("{}", &res[0]);

    println!("\n//6.表达式赋值==================");
    test6();
}

fn test1() {
    let arr = [1, 2, 3, 4];
    let arr:[i32; 4] = [1, 2, 3, 4];
    println!("{:?}", arr); //打印数组方法
    println!("===========");
    println!("{:#?}", arr);//打印数组方法
}

fn test2() {
    let tuple: (i32, i32, i32) = (1, 2, 3);
    println!("{:?}", tuple);
}

fn test3() {
    let tuple: (i32, i32, i32) = (1, 2, 3);
    let (x, y, z) = tuple;
    println!("{}", x);
}

fn test4() {
    let (x, y, z) = (1, 2, 3);
    println!("{}", x);
}

fn test5() -> [i32; 1] {
    [100]; // 有分号的话，表示是语句，语句是没有返回值的
    [100]  // 没有分号的话，表示是表达式，表达式是有返回值的，所以说表达式是可以做为函数返回值的
}

fn test6() {
    // let x = (let y = 1); //这种是不行的，因为后面的let语句是没有返回值的，必须用表达式赋值才行
    let x = {
        let y = 1;
        y
    };
    println!("{x}");
}