use mylib::factory::fridge::produce as fridge_produce;
use mylib::factory::tv::produce as tv_produce;
// use mylib::factory::*;
// 上述把 factory下面的所有模块都导入，就可以使用 fridge::produce() or tv::produce() 这种方式调用

fn main() {
    fridge_produce();
    tv_produce();
}
