use crate::moda::A;
use crate::moda::modb::modc;

mod moda {
    #[derive(Debug)]
    pub struct A {
        pub id: u32,
        name: String,
    }

    impl A {
        pub fn new(_id: u32) -> A {
            A{
                id: _id,
                name: String::new(),
            }
        }

        pub fn print(&self) {
            println!("{:#?}", self);
        }
    }

    pub fn testx() {
        println!("testx");
    }

    pub mod modb {
        pub fn print_B() {
            println!("B");
        }

        pub mod modc {
            pub fn print_C() {
                println!("C");
                super::print_B(); //访问上一级的函数用 super
                super::super::testx(); //支持多级 super 调用
            }
        }
    }
}


fn main() {
    A::new(1).print();
    modc::print_C();
}
