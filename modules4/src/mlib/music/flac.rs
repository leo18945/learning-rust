

pub fn flac() {
    println!("flac");
    crate::translate::translate(); //根目录是mlib，所以就从根目录往下写了
    super::super::translate::translate();
}