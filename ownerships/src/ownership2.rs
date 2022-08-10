// 带你了解 Rust 中的move, copy, clone
// https://juejin.cn/post/7046638487851761694

#[test]
pub fn main() {
    println!("\n//1.rust通过所有权机制来管理内存，编译器在编译时就会根据所有权规则对内存的使用进行检查");
    test1();

    println!("\n//2.堆和栈");
    test2();

    println!("\n//3.作用域");
    test3();

    println!("\n//32.测试String离开作用域调用drop方法");
    test32();

    println!("\n//4.String内存回收");
    test4();

    println!("\n//6.测试所有权的4种语义");
    test6();

    println!("\n//8.函数调用改变变量的所有权");
    test8();

    println!("\n//9.函数调用改变变量的所有权");
    test9();

    println!("\n//10.引用");
    test10();
}

//1.rust通过所有权机制来管理内存，编译器在编译时就会根据所有权规则对内存的使用进行检查
fn test1(){

}

//2.堆和栈
fn test2(){

}

//3.作用域，例如{}所限定的作用域
fn test3(){
    let x = 1;
    {
        let y = 2;
        println!("{}", y);
    }
    // println!("{}", y); // error: cannot find value `y` in this scope
}

//32.测试String离开作用域调用drop方法
fn test32(){
    let s1 = "luck".to_string();
    {
        let s2 = s1;
        println!("{s2}");
    }
    // println!("{s1}"); 在这里s1已经无效了，因为在这之前把 s1 的所有权 move 给了 s2
    // 否则的话，在57行的时候，调用 drop 释放内存后，这里再调用一次 drop 函数，会造成重复释放内存问题
}

//4.String内存回收
fn test4(){
    let mut s1 = String::from("1");
    // let a1 = format!("{:p}", &s1);
    // println!("s1 addr = {a1}");
    println!("s1 addr = {:p}", &s1);
    println!("s1 prt  = {:?}", s1.as_ptr());

    let s2 = String::from("1234567890123456"); //结论是，前后两个字符串加起来超过16个字符，就会生成新的Vec数据结构来存放

    println!("s2.len = {}", s2.len());

    s1.push_str(&s2);

    println!("after push ({}) string, s1 address", s2.len());
    println!("s1 addr = {:p}", &s1);
    println!("s1 prt  = {:?}", s1.as_ptr());
}
/**
========================
s1 addr = 0x7ffee05c10e0
s1 prt  = 0x7fd1eb405af0
s2.len = 16
after push (16) string, s1 address
s1 addr = 0x7ffee05c10e0
s1 prt  = 0x7fd1eb405b10
 */

fn test6(){
    println!("\n6.1->copy 语义");
    test61();

    println!("\n6.2->move 语义");
    test62();

    println!("\n6.3->clone 语义");
    test63();

    println!("\n6.4->borrow 语义");
    test64();
}

//6.1->copy 语义
fn test61(){
    let s1 = 1;
    println!("s1 addr -> {:p}", &s1);         // s1 addr -> 0x70000edb65a4
    println!("s1 context -> {s1}");           // s1 context -> 1

    let s2 = s1;                         // 在这个等号执行时，执行的是 copy, 这是因为这些基本数据类型使用copy复制起来成本比较低，所以就实现了 copy trait
    println!("after 'let s2 = s1'");          // after 'after 'let s2 = s1''
    println!("s2 addr -> {:p}", &s2);         // s2 addr -> 0x70000edb6634
    println!("s2 context -> {s2}");           // s2 context -> 1

    println!("s1 still alive -> {s1}");       // s1 still alive -> 1
}

//6.2->move 语义
fn test62(){
    let s1 = "good".to_string();
    println!("s1 addr -> {:p}", &s1);          // s1 addr -> 0x700005eba4e8
    println!("s1 ptr  -> {:p}", s1.as_ptr());  // s1 ptr  -> 0x7f9c2ce04170
    println!("s1 context -> {s1}");            // s1 context -> good

    // move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    let s2 = s1;                        // 在这个等号执行时，执行的是 move, 这是因为String类型底层是 Vec<u8> 类型在堆上分配内存，使用copy复制起来成本比较高，所以就没有实现了 copy trait，这样的等号赋值时就使用了 move 语义
    println!("after 'let s2 = s1'");           // after 'let s2 = s1'
    println!("s2 addr -> {:p}", &s2);          // s2 addr -> 0x700005eba600
    println!("s2 ptr  -> {:p}", s2.as_ptr());  // s2 ptr  -> 0x7f9c2ce04170 // 和 s1 ptr 地址相同，说明指向了同一块内存
    println!("s2 context -> {s2}");            // s2 context -> good

    // println!("s1 still alive -> {s1}");     // s1 invaild
    // 但此时通过 LLDB 还是可以读取到 s1 变量的值，只是通过代码不能再访问了
}

//6.3->clone 语义
fn test63(){
    let s1 = "good".to_string();
    println!("s1 addr -> {:p}", &s1);          // s1 addr -> 0x70000134b5a0
    println!("s1 ptr  -> {:p}", s1.as_ptr());  // s1 ptr  -> 0x70000134b648
    println!("s1 context -> {s1}");            // s1 context -> good

    let s2 = s1.clone();
    println!("after 'let s2 = s1.clone()'");  // after 'let s2 = s1.clone()'
    println!("s2 addr -> {:p}", &s2);         // s2 addr -> 0x70000134b690
    println!("s2 ptr  -> {:p}", s2.as_ptr()); // s2 ptr  -> 0x70000134b768 // 和 s1 ptr 地址不同，是因为重新分配了内存
    println!("s2 context -> {s2}");           // s2 context -> good

    println!("s1 still alive -> {s1}");       // s1 still alive -> good
}

//6.4->borrow 语义
fn test64(){
    let s1 = "good".to_string();
    println!("s1 addr -> {:p}", &s1);          // s1 addr -> 0x70000134b5a0
    println!("s1 ptr  -> {:p}", &s1.as_ptr()); // s1 ptr  -> 0x70000134b648
    println!("s1 context -> {s1}");            // s1 context -> good

    let s2 = s1.clone();
    println!("after 'let s2 = s1.clone()'");  // after 'let s2 = s1.clone()'
    println!("s2 addr -> {:p}", &s2);         // s2 addr -> 0x70000134b690
    println!("s2 ptr  -> {:p}", &s2.as_ptr());// s2 ptr  -> 0x70000134b768 // 和 s1 ptr 地址不同，是因为重新分配了内存
    println!("s2 context -> {s2}");           // s2 context -> good

    println!("s1 still alive -> {s1}");       // s1 still alive -> good
}

//8.函数调用改变变量的所有权
fn test8(){
    let s1 = "good".to_string();

    fn take_ownership(s: String) {
        println!("{s}");
    }

    take_ownership(s1);

    // println!("{s1}"); // value borrowed here after move
}

//9.函数调用改变变量的所有权
fn test9(){
    println!("9.1函数调用改变变量的所有权的问题现状");
    test91();

    println!("9.2使用引用改变-函数调用改变变量的所有权的问题");
    test92();
}

fn test91(){
    fn give_ownership() -> String {
        "good".to_string()
    }

    fn take_and_give_ownership(s: String) -> String {
        s
    }

    let s1 = give_ownership();
    let mut s2 = "good".to_string();
    let s3 = take_and_give_ownership(s2);
    s2 = take_and_give_ownership(s3); // 这里再次调用可以使用s2了，但又不能再使用s3了

    println!("s1->{}", s1);
    println!("s2->{}", s2); // 这里不能再使用 s2
    // println!("s3->{}", s3);
}

fn test92(){
    fn calc_len(s: &String) -> usize {
        s.len()
    }

    let s1 = "good luck".to_string();
    let len = calc_len(&s1);
    println!("len({}) -> {}", s1, len);
}
/**
let s1 = "good luck".to_string();
// 以上s1类型的变量，通过3层结构就可以访问到具体的字符串值->0x67 0x6f 0x6f 0x64 0x20 0x6c 0x75 0x63 0x6b
(lldb) p &s1
(*mut alloc::string::String) &s1 = 0x000070000e3866b0

(lldb) x/3gx 0x000070000e3866b0
0x70000e3866b0: 0x00007f831ec05310 0x0000000000000009
0x70000e3866c0: 0x0000000000000009

(lldb) x/9bx 0x00007f831ec05310
0x7f831ec05310: 0x67 0x6f 0x6f 0x64 0x20 0x6c 0x75 0x63
0x7f831ec05318: 0x6b

======================================================
s = &String，s这类的指针变量，在String类型变量的外面，再包装了一层指针结构，要通过4层才能访问到具体的字符串值，而String类型的变量只要3层，如下
(lldb) p &s
(*mut &alloc::string::String) &s = 0x000070000e386658

(lldb) x/3gx 0x000070000e386658
0x70000e386658: 0x000070000e3866b0 0x000070000e386730
0x70000e386668: 0x0000000102d36169

(lldb) x/3gx 0x000070000e3866b0
0x70000e3866b0: 0x00007f831ec05310 0x0000000000000009
0x70000e3866c0: 0x0000000000000009

(lldb) x/9bx 0x00007f831ec05310
0x7f831ec05310: 0x67 0x6f 0x6f 0x64 0x20 0x6c 0x75 0x63
0x7f831ec05318: 0x6b
*/

//https://rust-book.junmajinlong.com/ch6/03_ref_ownership_borrow.html
//https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html

fn test10() {
    println!("test101=================");
    test101();

    println!("test102=================");
    test102();

    println!("test103=================");
    test103();

    println!("test104=================");
    test104();

    println!("test105=================");
    test105();
}

fn test101() {
    fn calc_len(s: &String) -> usize {
        s.len()
    }

    fn modify_str(s: &mut String) {
        s.push_str("~~~~~~~~");
    }

    let mut s1 = "good".to_string();
    let s2 = &s1;
    let len = calc_len(s2);
    println!("len({}) = {}", s2, len);

    let s3 = &mut s1;
    modify_str(s3);

    println!("s1 = {}", s1);
}

fn test102() {
    fn modify_str(s: &mut String) {
        s.push_str("~~~~~~~~");
    }

    let mut s1 = "good".to_string();
    let s2 = &mut s1;
    modify_str(s2);

    // println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}

fn test103() {
    fn modify_str(s: &mut String) {
        s.push_str("~~~~~~~~");
    }

    let mut s1 = "good".to_string();
    modify_str(&mut s1);
    println!("s1 = {}", s1);
}

fn test104() {
    let mut s1 = "good".to_string();
    (&mut s1).push_str(" luck");
    println!("s1 = {}", s1);
}

fn test105() {
    let mut s1 = "good".to_string();
    let s2 = &mut s1;
    println!("s1={}", s1);
    // println!("s2={}", s2);
    // s1 和 s2 不能同时存在
}