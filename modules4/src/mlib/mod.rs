pub mod translate;
pub mod music;
mod init;

pub fn version() {
    println!("ver:1.0.0");
}

//把 flac 方法导入到这个位置，就可以通过 mlib::flac(); 进行调用
pub use music::flac::flac;