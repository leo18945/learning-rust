



fn main() {
    println!("\n//1.定义结构体");
    test1();

    println!("\n//2.创建结构体实例");
    test2();

    println!("\n//3.修改结构体字段");
    test3();

    println!("\n//4.参数名字和字段名字同名的简写方法");
    test4();

    println!("\n//5.从其它结构体创建实例");
    test5();

    println!("\n//6.元组结构体");
    test6();

    println!("\n//7.没有任何字段的类单元结构体");
    test7();

    println!("\n//8.打印结构体");
    test8();

    println!("\n//9.结构体添加成员方法");
    test9();

    println!("\n//10.结构体封装 getter 方法");
    test10();

}

//1.定义结构体
fn test1() {
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }
}

//2.创建结构体实例
fn test2() {
    #[derive(Debug)]
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }

    // 非 mut 变量是不能修改的
    let user = User{
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };
    println!("{:#?}", user);
}

//3.修改结构体字段
fn test3() {
    #[derive(Debug)]
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }
    // 这里要定义成 mut 变量，才能修改
    let mut user = User{
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };
    println!("{:#?}", user);
    user.name = "jack".to_string();
    println!("{:#?}", user);
}

//4.参数名字和字段名字同名的简写方法
fn test4() {
    #[derive(Debug)]
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }
    let id = 101;
    let name = "leo".to_string();
    let is_active = true;
    let user = User{
        id, //这里因为 参数名字和字段名字同名，所以就只用写一次
        name,
        is_active,
    };
    println!("{:#?}", user);
}

//5.从其它结构体创建实例
fn test5() {
    #[derive(Debug)]
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }

    let leo = User{
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };

    let jack = User{
        name: "jack".to_string(),
        ..leo
    };
    println!("{:#?}", jack);
}

//6.元组结构体
fn test6() {
    //元组结构体
    //1.圆括号代替花括号
    //2.字段没有名字，只有类型
    #[derive(Debug)]
    struct Point(i32, i32);

    let p = Point(201, 197);
    println!("{:#?}", p);
    println!("x={}, y={}", p.0, p.1); //通过元组tuple的方式引用
}

//7.没有任何字段的类单元结构体
fn test7() {
    #[derive(Debug)]
    struct Serializable;

    let s = Serializable;
    println!("{:#?}", s);
}


//8.打印结构体
fn test8() {
    #[derive(Debug)]
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }

    // 非 mut 变量是不能修改的
    let user = User{
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };
    println!("{:#?}", user);
}

//9.结构体添加成员方法
fn test9() {
    #[derive(Debug)]
    struct Serializable{}

    impl Serializable {
        pub fn print(&self) { //成员方法一定要加 &self, 返回本类型用 -> Self
            println!("{self:p}"); //打印变量自身地址
        }
    }

    let s = Serializable{};
    s.print();
}

//10.结构体封装 getter 方法
fn test10() {
    #[derive(Debug)]
    struct User {
        id  : u32,
        name: String,
        is_active: bool,
    }

    impl User {
        pub fn get_id(&self) -> u32 { //成员方法一定要加 &self, 返回本类型用 -> Self
            self.id
        }
    }
    // 同一个结构体的多个方法，可以分开多次实现，不像java必须在同一个class里面写完
    impl User {
        pub fn get_name(&self) -> &str { //成员方法一定要加 &self, 返回本类型用 -> Self
            self.name.as_ref()
        }
    }

    let user = User{
        id: 101,
        name: "leo".to_string(),
        is_active: true,
    };

    println!("{}", &user.get_id());
    println!("{}", &user.get_name());

}