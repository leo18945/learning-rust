use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use ahash::{AHasher, RandomState};

fn main() {
    println!("1.====用 String 类型做 key ========");
    test1();

    println!("2.====用 &String 类型做 key ========");
    test2();

    println!("3.====用字符串字面量类型&str做 key ========");
    test3();

    println!("4.====用字符串字面量类型&str做 key ========");
    test4();

    println!("5.====批量初始化 k->v========");
    test5();

    println!("6.====通过两个 vec zip 实现批量初始化 hashmap ========");
    test6();

    println!("7.====打印map.len & cap========");
    test7();

    println!("8.====通过 for (k, v) in map 遍历 map ========");
    test8();

    println!("9.====只在key 不存在时才插入 ========");
    test9();

    println!("10.====word count 实例 ========");
    test10();

    println!("11.====替换 hasher 函数，获得更高性能 ========");
    test11();
}

fn test1() {
    let mut h1: HashMap<String, i32> = HashMap::new();
    h1.insert(String::from("st1"), 1);
    h1.insert(String::from("st2"), 2);
    h1.insert(String::from("st3"), 3);

    println!("{:#?}", h1);

    let v1 = h1.get(&String::from("st3")).unwrap();
    println!("{}", v1);
}

fn test2() {
    let mut map = HashMap::new();
    let k1 = String::from("k1");
    let k2 = String::from("k2");
    let k3 = String::from("k3");
    map.insert(&k1, 1);
    map.insert(&k2, 2);
    map.insert(&k3, 3);

    let v1 = map.get(&k1).unwrap();
    println!("{}", v1);

    let v2 = map.get(&k2).unwrap();
    println!("{}", v2);

    let v3 = map.get(&k3).unwrap();
    println!("{}", v3);
}

fn test3() {
    let mut map = HashMap::new();
    let k1 = "k1";
    let k2 = "k2";
    let k3 = "k3";
    map.insert(k1, 1);
    map.insert(k2, 2);
    map.insert(k3, 3);

    let v1 = map.get(k1).unwrap();
    println!("{}", v1);

    let v2 = map.get(k2).unwrap();
    println!("{}", v2);

    let v3 = map.get(k3).unwrap();
    println!("{}", v3);
}

fn test4() {
    let mut map = HashMap::new();
    map.insert("k1", 1);
    map.insert("k2", 2);
    map.insert("k3", 3);

    let v1 = map.get("k1").unwrap();
    println!("{}", v1);

    let v2 = map.get("k2").unwrap();
    println!("{}", v2);

    let v3 = map.get("k3").unwrap();
    println!("{}", v3);
}

fn test5() {
    // 批量初始化 k->v
    let mut map = HashMap::from([("k1", "v1"), ("k2", "v2"), ("k3", "v3")]);
    println!("{:?}", map);

    println!("k1->{}", map.get("k1").unwrap());
    println!("k2->{}", map.get("k2").unwrap());
    println!("k3->{}", map.get("k3").unwrap());
}

fn test6() {
    let mut keys = vec!["k1", "k2", "k3"];
    let mut vals = vec!["v1", "v2", "v3"];
    let map: HashMap<_, _> = keys.iter().zip(vals.iter()).collect();
    println!("map => {:#?}", map);
    println!("k1->{}", map.get(&"k1").unwrap());
}

fn test7() {
    let mut map = HashMap::from([("k1", "v1"), ("k2", "v2"), ("k3", "v3")]);
    println!("{:?}", map);
    println!("len={}, cap={}", map.len(), map.capacity());
}

fn test8() {
    let mut map = HashMap::from([("k1", "v1"), ("k2", "v2"), ("k3", "v3")]);
    println!("{:?}", map);
    println!("len={}, cap={}", map.len(), map.capacity());
    for (k, v) in &map { //遍历的时候，它会以任意的顺序输出
        println!("{}->{}", k, v);
    }
}

fn test9() {
    let mut map = HashMap::from([("k1", "v1"), ("k2", "v2"), ("k3", "v3")]);
    map.entry("k2").or_insert("v22");//存在则不插入
    map.entry("k4").or_insert("v4"); //不存在则插入
    println!("{:#?}", map);
}

fn test10() {
    let line = "hello world, hello rust, it's cool";
    let mut map = HashMap::new();
    for word in line.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert 没插入返回之前旧值，插入则返回新值
        *count += 1; //对上述返回值加1
    }
    println!("{:#?}", map);
}

fn test11() {
    // https://github.com/tkaitchuck/ahash
    // https://crates.io/crates/ahash
    let mut map: HashMap<&str, &str, RandomState> = HashMap::default();
    map.insert("k1", "v1");
    map.insert("k2", "v2");
    map.insert("k3", "v3");
    println!("{:#?}", map);
}