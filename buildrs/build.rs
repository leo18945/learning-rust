use std::{env, fs};
use std::path::Path;

fn main() {
    // env::set_var("OUT_DIR", "/Users/leo18945/CLionProjects/rust/learning-rust/buildrs/src");
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("/math.rs");
    // let dest_path = Path::new("/Users/leo18945/CLionProjects/rust/learning-rust/buildrs/src/math.rs");
    let dest_path = Path::new("/Users/leo18945/math.rs");
    fs::write(dest_path,
        "\
        pub fn max2(a: i32, b: i32) -> i32 {\
            if a > b {a}\
            else {b}\
        }\
        "
    ).unwrap();
    // 通过 println 输出 cargo:rerun-if-changed=build.rs 内容，给 cargo 下达指令
    // 指令内容就是 println 输出的内容，在此处的意思是：当文件修改后重新运行
    println!("cargo:rerun-if-changed=build.rs");
}