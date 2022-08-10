
//1.Rust中一个实现消息传递并发的主要工具是通道，通道由两部分组成，一个是发送端，一个是接收端，发送端用来发送消息，接收端用来接收消息。
//  发送者或接收者任一个被丢弃时就可以认为通道被关闭了

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::prelude::Rng;

//2.通道常用方法
//1) 通过 mpsc::channel 创建通道，mpsc 是多个生产者，单个消费者
//2) 通过 spmc::channel 创建通道，spmc 是一个生产者，多个消费者
//3) 创建通道后返回的是发送者和消费者一对变量
fn main() {
    println!("\n1.构建 channel ");
    test1();

    // println!("\n2.构建 channel ");
    // test2();
    //
    // println!("\n2.构建 channel ");
    // test3();
}

fn test1() {
    // println!("====test11==================");
    // test11();
    // println!("====test12==================");
    // test12();
    println!("====test13==================");
    test13();
}

fn test11() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello world".to_string()).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("rec->{}", msg);

    // 为什么要这里不需要handle.join，也可以保证子线程发送完后，主线程收到消息
    // handle.join().unwrap();
    // 这是因为 rx.rec 方法是阻塞式的，没收到消息不会往下执行
    // 如果不想要阻塞的，可以用 rx.try_recv 这个非阻塞式的
}

fn test12() {
    let (tx, rx) = mpsc::channel();

    tx.send("hello world".to_string()).unwrap();

    thread::spawn(move || {
        let msg = rx.recv().unwrap();
        println!("rec->{}", msg);
    });
}

fn test13() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello world".to_string()).unwrap();
    });

    let msg = rx.try_recv(); // 用 try_recv 就不会阻塞了，主线程直接返回，就没收到消息
    if let Ok(v) = msg {
        println!("rec->{}", v);
    }
}

fn test2() {
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        tx.send("hello rust".to_string()).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let msg = rx.recv().unwrap();
        println!("msg={}", msg);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn test3() {
    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let mut gen = rand::thread_rng();
        loop {
            let n: i32 = gen.gen();
            println!("p->{}", n);
            tx.send(n).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    let handle2 = thread::spawn(move || {
        loop {
            let n = rx.recv().unwrap();
            println!("c->{}", n);
            thread::sleep(Duration::from_secs(1))
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}