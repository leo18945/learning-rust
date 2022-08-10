use std::env;
// include!(concat!(env!("OUT_DIR"), "/math.rs"));
include!("/Users/leo18945/math.rs");

fn main() {
    // 通过 build.rs 生成 max2 函数，然后在此处调用
    let m = max2(56, 47);
    println!("max2(56, 47) -> {}", m);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("{:?}", out_dir);
}

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

info!("This is version {}{}, built for {} by {}.",
      built_info::PKG_VERSION,
      built_info::GIT_VERSION.map_or_else(|| "".to_owned(),
                                          |v| format!(" (git {})", v)),
      built_info::TARGET,
      built_info::RUSTC_VERSION);

trace!("I was built with profile \"{}\", features \"{}\" on {} using {}",
       built_info::PROFILE,
       built_info::FEATURES_STR,
       built_info::BUILT_TIME_UTC,
       built_info::DEPENDENCIES_STR);