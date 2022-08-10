mod conf;
mod environment;
mod errorr;

// https://github.com/ZhangHanDong/inviting-rust
#[allow(unused)]
fn main() {
    let conf = conf::PoemConfig::read_config();
    println!("{:#?}", conf);
}
