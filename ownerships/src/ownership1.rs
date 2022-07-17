use std::mem;
use std::mem::{align_of, size_of};

#[test]
pub fn main() {
    println!("\n1.==========================");
    test1();

    println!("\n2.==========================");
    test2();

    println!("\n3.==========================");
    test3();
}

pub fn test1() {
    println!("Ownership");
    let mut a = String::from("a");
    println!("{}, {}", a.capacity(), a.len());

    let mut a = String::new();
    println!("{}, {}", a.capacity(), a.len());

    let b = 1;
    let c = b;
    println!("b={}, c={}", b, c);

    let d = String::from("hello d");
    let e = d; // d已死
    // println!("d={}, e={}", d, e); // 不能打印已死的变量

    take_ownership(e); // e 已死
    // println!("{}", e); // 此处不能再打印 e

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn take_ownership(str1: String) {
    println!("{str1}");
}

fn gives_ownership() -> String {
    let str1 = String::from("hello");
    str1
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}

////////////////////////////////////////////
fn test2() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    println!("{:p}", &data);
    println!("{:p}", &&data);
    println!("{:p}", data1);
    println!("2.sum of data1: {}", sum(&data));
    // println!("2.sum of data1: {}", sum(data1));
    // println!("{:p}", &data1);
    // println!("{:p}", &data[0]);

    // println!(
    //     "1.addr of value: {:p}({:p}), addr of data: {:p}, data1: {:p}",
    //     &data, data1, &&data, &data1
    // );

    // println!("2.sum of data1: {}", sum(data1));
    //
    // println!(
    //     "3.addr of items: [{:p}, {:p}, {:p}, {:p}]",
    //     &data[0], &data[1], &data[2], &data[3]
    // )
}

fn test3() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;

    // println!("var1 size = {}", mem::size_of_val(data));
    // println!("data1 size = {}", mem::size_of::<data1>()); //24
    println!("data1 size = {}", mem::size_of_val(data1)); //24

    let hello = String::from("Hello, world!");
    let p1 = hello.as_ptr();

    println!("test3-> &data={:p}", &data);    //0x7ffee88a9208
    println!("test3->&&data={:p}", &&data);   //0x7ffee88a92b8

    println!("test3-> data1={:p}", data1);    //0x7ffee88a9208
    println!("test3->&data1={:p}", &data1);    //0x7ffee88a9208

    println!("test3->&data[0]={:p}", &data[0]); //0x7fb860c05af0
    println!("2.sum of data1: {}", sum(&data)); //2.sum of data1: 10

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn sum(data: &Vec<i32>) -> i32 {
    println!("sum->data={:p}", data);    //0x7ffee88a9208
    println!("sum->&data={:p}", &data);   //0x7ffee88a9010
    println!("sum->&&data={:p}", &&data);  //0x7ffee88a90e8

    println!("sum->&data[0]={:p}", &data[0]);//0x7fb860c05af0
    println!("x.addr of value: {:p}, addr of ref: {:p}", data, &data); //x.addr of value: 0x7ffee5ba0250, addr of ref: 0x7ffee5ba00b8
    data.iter().fold(0, |acc, x|acc + x)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}