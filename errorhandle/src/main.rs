use std::fs::File;
use std::io;
use std::io::Read;

const FILE_NAME: &str = "/Users/leo18945/CLionProjects/rust/learning-rust/Cargo.toml";

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

fn test1() {
    let f = File::open(FILE_NAME);
    let r = match f {
        Ok(mut filexxx) => {
            let mut s = String::new();
            filexxx.read_to_string(&mut s);
            println!("file content=\n{}", s);
        },
        Err(error) => panic!("error: {:?}", error)
    };
}

fn test2() {
    fn read_file() -> Result<String, io::Error> {
        let mut f = File::open(FILE_NAME)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    let res = read_file();
    match res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }
}

fn test3() {
    // 推荐用这种方式写
    fn read_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open(FILE_NAME)?.read_to_string(&mut s)?;
        Ok(s)
    }

    let res = read_file();
    match res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }
}

fn test4() {
    fn read_file() -> Result<String, io::Error> {
        let mut s = String::new();
        let mut f = File::open(FILE_NAME).unwrap();
        f.read_to_string(&mut s);
        Ok(s)
    }

    let res = read_file();
    match res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }
}

fn test5() {
    fn read_file() -> Result<String, io::Error> {
        let mut s = String::new();
        let mut f = File::open(FILE_NAME).expect("No no no no no, file not found!");
        f.read_to_string(&mut s);
        Ok(s)
    }

    let res = read_file();
    match res {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }
}