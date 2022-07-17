// #![feature(drain_filter)]
fn main() {
    println!("========================");
    test1();

    println!("========================");
    test2();

    println!("========================");
    test3();

    println!("========================");
    test4();

    println!("========================");
    test5();

    println!("========================");
    test6();

    println!("========================");
    test7();

    println!("\n8.验证读写引用不能重复使用========================");
    test8();

    println!("\n9.用 remove 删除单个元素========================");
    test9();

    println!("\n10.用 retain 指定条件-批量删除元素-用例1========================");
    test10();

    println!("\n11.用 retain 指定条件-批量删除元素-用例2========================");
    test11();

    println!("\n12.用 drain_filter 指定条件-批量删除元素========================");
    test12();
}

fn test1() {
    println!("Hello, world!");
    let mut v = vec![1, 2, 3, 4, 5];
    v.iter().for_each(|i| println!("{i}"));
}

fn test2() {
    let mut v = vec![1, 2, 3, 4, 5];
    let a = &v[1];
    println!("{a}");
}

fn test3() {
    let mut v = vec![1, 2, 3, 4, 5];
    match v.get(9) { // 用这种方式比较好，可以处理数组下标越界的问题
        Some(val) => println!("{val}"),
        _ => println!("no value")
    }
}

fn test4() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for a in v {
        let q = &a;
        println!("{q:p}->{a}"); //a的地址都一样，因为是a的值是v[n]元素的副本
    }
}

fn test5() {
    let mut v: Vec<i32> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    for a in &v {
        println!("{a:p}->{a}");  //a的地址都不一样，因为是a的值就是指向v[n]元素的地址
    }
}

fn test6() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for a in &mut v {
        *a *= 2;
        println!("{a:p}->{a}"); //a的地址都一样，因为是a的值是v[n]元素的副本
    }
}

fn test7() {

    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }

    // 把不同类型的变量放在同一个 Vec 里面，可以用枚举来实现
    let mut v = vec![
        Context::Text(String::from("Text")),
        Context::Float(1.23),
        Context::Int(100),
    ];

    for i in &v {
        println!("{:?}", i);
    }
}

fn test8() {
    println!("\ntest81-测试用 push 增加元素=========================");
    test81();

    println!("\ntest82-测试用 extend_from_slice 批量增加元素=========================");
    test82();

    println!("\ntest83-测试为什么push后first变量就用不了？是因为push后first指向的内存地址变了访问不到了=========================");
    test83();
}

fn test81() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6); //对v执行push方法后，有可能会重新分配内存空间，以前的内存有可能不够用
    //那即然都重新分配了内存，first指针指向的内存就没有意义了，所以后继就不允许再用
    // println!("first={first}");
    println!("v={:?}", v);
}

fn test82() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    let first = &v[0];
    v.extend_from_slice(&[6, 7, 8, 9, 10]); //批量增加元素
    println!("v={:?}", v);
}

fn test83() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    println!("v addr = {:p}", &v);
    let vptr1 = format!("{:?}", v.as_ptr());
    println!("v prt  = {vptr1}");

    let first = &v[0];
    println!("first addr = {:p}", &first); //输出变量地址，这个地址中存储着一个指针值
    println!("first ptr  = {:p}", first);  //输出变量值-因为是指针型变量，所以值是一个内存地址，这个地址是指向vec数组在堆上的内存首地址

    // v.extend_from_slice(&[6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    for i in 6..16 { //6..16是前闭后开，也就是不包括16
        v.push(i);
    }
    println!("after push some elements");
    println!("v={:?}", v);
    println!("v addr = {:p}", &v);
    let vptr2 = format!("{:?}", v.as_ptr());
    println!("v prt  = {vptr2}");
    println!("========summary========");
    println!("old ptr={}", vptr1);
    println!("new ptr={}", vptr2);
    if vptr1 == vptr2 {
        println!("they are the same!");
    } else {
        println!("they are not the same! the Vec array has been allocate new memory.");
    }

    // v.push(6); //对v执行push方法后，有可能会重新分配内存空间，以前的内存有可能不够用
    //那即然都重新分配了内存，first指针指向的内存就没有意义了，所以后继就不允许再用
    // println!("first={first}");
}

fn test9() {
    let mut v = vec![1, 2, 3, 4, 5];

    // error[E0658]: use of unstable library feature 'drain_filter': recently added
    // v.drain_filter(|x| *x % 2 == 0);
    println!("删除前元素->{:?}", v);

    println!("删除前地址 v prt  = {:?}", v.as_ptr());

    v.remove(2);
    println!("删除后元素->{:?}", v);
    println!("删除后地址 v prt  = {:?}", v.as_ptr()); //地址没有发生改变
    // 但元素往前填充，后面的元素在内存中还是有值，只是读取不到了，通过 len 限制了，cap 还是没变
}

fn test10() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.retain(|x| *x % 2 == 0);
    // v.retain(|&x| x % 2 == 0);
    println!("{:?}", v);
}

fn test11() {
    let mut v = vec![1, 2, 3, 3, 4, 5];

    v.retain(|x| *x != 3);
    // v.retain(|&x| x % 2 == 0);
    println!("{:?}", v);
}

fn test12() {
    let mut v = vec![1, 2, 3, 4, 5];

    println!("删除前元素->{:?}", v);
    println!("删除前地址 v prt  = {:?}", v.as_ptr());

    // error[E0658]: use of unstable library feature 'drain_filter': recently added
    // v.drain_filter(|x| *x % 2 == 0); // drain_filter 在这个版本中是试验性功能，在这个版本(stable)中使用不了，在未来版本中可以用

    println!("删除后元素->{:?}", v);
    println!("删除后地址 v prt  = {:?}", v.as_ptr()); //地址没有发生改变
}