
trait Animal {
    fn name(&self) -> &'static str;
    fn cry(&self);
}

struct Dog {
    name: &'static str,
    color: &'static str
}

impl Dog {
    fn cry(&self) {
        println!("woof woof woof");
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        self.name
    }

    fn cry(&self) {
        println!("woof woof");
    }
}

struct TibetanMastiff {
    dog: Dog,
    level: u8,
}

impl TibetanMastiff {
    pub fn fight(&self) {
        println!("fight so hard!");
    }
}

fn main() {
    test1();
    test2();

}

fn test1() {
    let animal: &dyn Animal = &Dog { name: "dog", color: "black" };
    // animal.fight();
    animal.cry();

    let dog = TibetanMastiff{dog: Dog{name: "dog", color: "black"}, level: 1};
    dog.fight();
    dog.dog.name();
}

fn test2() {
    trait A{
        fn a(&self){println!("from A");}
    }

    trait X{
        fn x(&self){println!("from X");}
    }

    // 类型B同时实现trait A和trait X
    // 类型B还定义自己的方法b
    struct B{}
    impl B {fn b(&self){println!("from B");}}
    impl A for B{}
    impl X for B{}

    fn run(){
        // bb是A的Trait Object实例，
        // bb保存了指向类型B实例数据的指针和指向vtable的指针
        let a: &dyn A = &B{};
        a.a();  // 正确，bb可调用实现自Trait A的方法a()
        // a.x();  // 错误，bb不可调用实现自Trait X的方法x()
        // a.b();  // 错误，bb不可调用自身实现的方法b()
    }
    run();

}