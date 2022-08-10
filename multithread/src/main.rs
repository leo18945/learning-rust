use std::thread;
use std::time::Duration;

/**
1.进程是资源分配的最小单位，线程是CPU调度的最小单位
2.在使用多线程时，经常会遇到的一些问题：
    1)竟争状态：多个线程以不一致的顺序访问数据或资源
    2)死锁：两个线程相互等待对方停止使用其所拥有的资源，造成两者都永久等待;
        A线程: 1->2->3 B线程:2->1->3 在t1时刻-> A:1,B:2  接下来等待： A:2,B:1
    3)只会发生在特定情况下且难以稳定重现和修复的bug
3.编程语言提供的线程叫做绿色线程，如go语言，在底层实现了M:N的模型，好M个绿色线程对应N个OS线程，但是，Rust标准库只提供1:1的线程模型的实现，即一个Rust线程对应一个OS线程

另：运行时代表二进制的文件中包含的由语言本身提供的代码，这些代码根据语言的不同可大可小，不过非汇编语言都会有一定数量的运行时代码。
通常，大家说一个语言“没有运行时”，是指这个语言的运行时“很小”。Rust, C都是几乎没有运行时的。
*/
fn main() {
    println!("\n1.测试主线程不会等待子线程执行完，就先行结束整个程序退出，此时子线程还有代码没执行完(没用join)");
    test1();

    println!("\n2.测试线程在执行主线程代码后join，主线程等待子线程执行完再结束，并且子线程和主线程代码并行执行");
    test2();

    println!("\n3.测试线程在执行主线程代码前join，主线程等待子线程执行完再执行，并且子线程和主线程代码串行执行");
    test3();

    println!("\n4.线程要访问闭包外部变量时的move语法");
    test4();

    println!("\n5.另一种方式构造线程");
    test5();
}

//主线程不会等待子线程结束，就提前结束了
fn test1() {
    test11();
    test12();

    for i in 0..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Hello, world!");
}

fn test11() {
    thread::spawn(||{
        for i in 0..10 {
            println!("sub1 thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

fn test12() {
    thread::spawn(||{
        for i in 0..10 {
            println!("sub2 thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

fn test2() {
    let handle = thread::spawn(||{
        for i in 0..10 {
            println!("sub1 thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("sub1 thread finish...");
    });

    for i in 0..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("main thread finish...");
    handle.join().unwrap();
    // 参考 C 语言实现
    // http://c.biancheng.net/view/8628.html
}

fn test3() {
    let handle = thread::spawn(||{
        for i in 0..10 {
            println!("sub1 thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("sub1 thread finish...");
    });

    // join 放在这里的话，就要等到这句话之前的所有代码执行完了，才会执行后继的代码
    // 而不是后面的代码和前面的子线程代码同时运行，就成串行了
    handle.join().unwrap();

    for i in 0..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("main thread finish...");
}

fn test4() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move||{
        // thread::sleep(Duration::from_secs(10));
        println!("v{:?}", v);
    });
    // drop(v);
    // 类似于在这里drop了变量v，但在子线程中休眠了10秒后再用v的时候，发现v已经不存在了
    // 为了防止以上情况发生，rust要求如果在子线程中要使用主线程中的变量时，必须把所有权转移(move)到子线程中，这样变量就与主线程无关了，就不会提前释放
    handle.join().unwrap();
}

fn test5() {
    let handle = thread::Builder::new()
        .name("sub thread 1".to_string())
        .stack_size(1024 * 2)
        .spawn(|| {
            println!("sub thread 1 started");
        });
    handle.unwrap().join().unwrap();
}