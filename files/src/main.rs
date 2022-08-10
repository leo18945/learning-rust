#[allow(unused_imports)]

// #![cfg_attr(dev, allow(unused_imports))]
// #![cfg_attr(dev, allow(dead_code))]
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![cfg(dev)]
// #![allow(unused_imports)]
// #![allow(dead_code)]

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::Path;
use chrono::prelude::*;
use fake::{Fake, Faker};
use fake::faker::lorem::en::Sentence;

const FILE: &str = "out.txt";
const FILE2: &str = "out3.txt";

fn main() {
    // test11();

    test12();

    test13();

    test14();

    test21();

    test22();
}

fn test11() -> std::io::Result<()> {
    println!("\n######## test11 -> create file");
    let mut file = File::create(FILE)?;
    file.write_all(b"Hello Rust!");
    let len = file.metadata().unwrap().len();
    println!("file len={}", len);
    Ok(())
}

fn test12() -> std::io::Result<()> {
    println!("\n######## test12 -> open file");
    let mut file = File::open(FILE)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}

fn test13() {
    println!("\n######## test13 -> open or create and write or append file");
    let mut file = OpenOptions::new()
        .create(true)
        // .create_new(false)
        // .read(true)
        .write(true)
        .append(true)
        .open(FILE2)
        .unwrap();

    let str = gen_fake_line();
    file.write_all(str.as_bytes());
    file.write_all(b"\n");

    let utc = Utc::now();
    // let time: String = utc.to_rfc2822();
    let time: String = utc.to_string();
    file.write_all(time.as_bytes());
    file.write_all(b"\n");
    file.flush();

    // file.rewind();
    // let mut content = String::new();
    // file.read_to_string(&mut content);
    // for line in content.lines() {
    //     println!("{}", line);
    // }
}

// https://github.com/cksac/fake-rs
// https://morioh.com/p/98815cb2717f
fn gen_fake_line() -> String {
    println!("=========================");
    use fake::faker::lorem::en::*;
    let s: String = Sentence(10..40).fake();
    // println!("{}", s);
    s
}

fn test14() {
    println!("\n######## test14 -> another read file lines.");
    let content = fs::read_to_string(FILE2).unwrap();
    for line in content.lines() {
        println!("{}", line);
    }
}

// #[test]
fn test21() {
    println!("\n######## test21 -> get file path");
    let path = Path::new(FILE);
    let full_path = path.file_name().unwrap();
    println!("{:?}", full_path);
}

fn test22() {
    println!("\n######## test22 -> iterate current dir.");
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("{:?} is a dir", path);
        } else {
            println!("{:?} is a file", path);
        }
    }
}