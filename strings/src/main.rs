fn main() {
    test1();
    println!("=================");
    test2();
    println!("=================");
    test3();
    println!("=================");
    test4();
    println!("=================");
    test5();
    println!("=================");
    test6();
    println!("=================");
    test7();
    println!("====用 nth 获取 chars中单个字符=============");
    test8();
    println!("====用 chars 处理String=============");
    test9();
    println!("====用 bytes 处理String=============");
    test10();
    println!("====String & str=============");
    test11();
}

fn test1() {
    let mut s1 = String::new();
    let mut s1 = "s1".to_string();
    let mut s1 = String::from("s1");
    s1.push('2');
    s1.push_str(", good");
    let mut s2 = String::from("s2");
    let s3 = s1 + &s2; //加操作时，后面的变量要取地址
    println!("{s3}")
}

fn test2() {
    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2; //s1所有权move给了s3
    println!("s3={}", s3);
    // println!("s1={}", s1); //在这里时s1已不存在
    println!("s2={}", s2);
}

fn test3() {
    let s1 = "hello".to_string();
    println!("{}", s1);
    let mut v1: Vec<u8> = Vec::new();
    v1.push(0x68);
    v1.push(0x65);
    v1.push(0x6c);
    v1.push(0x6c);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.push(0x6f);
    v1.reserve(100);
    println!("{}", v1[0]);
}

//字符串按模板拼接
fn test4() {
    let s1 = String::from("apple");
    let s2 = String::from("banana");
    let s3 = String::from("pear");
    let s4 = format!("{}-{}-{}", s1, s2, s3); //这句执行完后，s1-s3还可以使用
    println!("s4={s4}");
    println!("s1={s1}");
    println!("s2={s2}");
    println!("s3={s3}");
}

fn test5() {
    let s1 = String::from("apple");
    // let i1 = s1[0]; String类变量不能通过索引进行访问，为什么？
    println!("apple.len={}", s1.len()); //len(apple)=5

    let s1 = String::from("索引");
    println!("索引.len={}", s1.len()); //len(索引)=6
    //len(索引)=6是因为String底层是用 UTF-8 来编码的，所以一个中文字符占3个字节
    //所以，当遇到s1[0]这种情况，就不知道怎样才能返回三分之一个字符了

    let t1 = &s1[0..3]; //像这种切片又可以，取3个字符就可以拿到一个汉字了
    println!("t1={}", t1);

    //let t1 = &s1[0..2]; //切片两个元素的话，会 panic
    //println!("t1={}", t1);
}

fn test6() {
    // 全英文字符的用法，有中文不能这样用
    let s1 = String::from("good");
    let bs1 = s1.as_bytes();
    let c1 = bs1[0] as char;
    let c2 = bs1[1] as char;
    println!("c1={}", c1); //g
    println!("c2={}", c2); //o
}

fn test7() {
    let s1 = String::from("这里不能用 1，因为nth背后有next动作");
    let mut cs1 = s1.chars();
    let mut count = cs1.count();
    println!("count={count}");
    // let c1 = &cs1.nth(0).unwrap(); // 这里为什么不能用了？
    // let c2 = cs1.nth(0).unwrap(); //这里不能用 1，因为nth背后有next动作
    // println!("c1={}", c1); //索
    // println!("c2={}", c2); //引
}

fn test8() {
    let s1 = String::from("这里不能用 1，因为nth背后有next动作");
    let mut cs1 = s1.chars();
    let c1 = cs1.nth(4).unwrap();
    println!("c1={}", c1); //用

    let c2 = cs1.nth(1).unwrap(); //这里不能用 1，因为nth背后有next动作
    println!("c2={}", c2); //1
}

fn test9() {
    let s1 = String::from("这里不能用 1，因为nth背后有next动作");
    let mut cs1 = s1.chars();
    for c in cs1 {
        println!("{}", c);
    }
}

fn test10() {
    let s1 = String::from("这里不能用 1，因为nth背后有next动作");
    let mut cs1 = s1.bytes();
    for c in cs1 {
        println!("{}", c);
    }
}

fn test11() {
    let s = String::from("hello world");
    let h = &s[0..5];  //这种写法也可以，是左闭右开
    let h = &s[..5];   //这种写法也可以
    let h = &s[0..=4]; //这种写法也可以，是左闭右闭
    let w = &s[6..11];
    let w = &s[6..=10];
    let w = &s[6..];
    let w = &s[..]; //取整个字符串
    // println!("{h}");
    // println!("{w}");
    let h1 = "hello";
    let w1 = "world";
    // let s2 = h.chars();
    println!("{}", h1.len());
}