mod ownership1;
mod ownership2;

fn main() {
    println!("owner ship test!");
    ownership1::test1();
    // main(); //这里居然还允许自已调自已，造成死循环
    // ownership1::main(); //另一mod中的main函数不允许这样调用
    // ownership2::main();
}
