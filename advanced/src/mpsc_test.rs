use crate::helpers::logger::init_logger;
use log::info;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test_mpsc_basic() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();

        // 第二条消息在通道中，如果没有第二个rx.recv，则不会消费到这条消息
        tx.send(String::from("world")).unwrap();
    });

    println!("before rx recv message");
    let result = rx.recv().unwrap();
    println!("Got: {}", result);
}

#[test]
fn test_mpsc_tx_drop() {
    let (tx, rx) = mpsc::channel::<String>();

    drop(tx);

    if let Ok(msg) = rx.recv() {
        println!("Got: {}", msg);
    }

    // let result = rx.recv().unwrap();
    // println!("Got: {}", result);
}

// 一直阻塞等待消息
#[test]
fn test_mpsc_wait_message() {
    init_logger();
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        for index in 0..10 {
            tx.send(format!("hello {}", index)).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    while let Ok(msg) = rx.recv() {
        info!("{}", format!("Got: {}", msg));
    }
}

// 使用loop的方式等价于while let的写法
#[test]
fn test_mpsc_wait_message_by_loop() {
    init_logger();
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        for index in 0..10 {
            tx.send(format!("nice to meet {}", index)).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    loop {
        if let Ok(msg) = rx.recv() {
            info!("Got: {}", msg);
        } else {
            println!("tx finished");
            break;
        }
    }
}

// 最多等待一段时间，超时则结束等待
#[test]
fn test_recv_timeout() {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send(String::from("hello world")).unwrap();
    });

    println!("waiting for message....");

    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(msg) => {
            println!("Got: {}", msg);
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            println!("tx timeout");
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            println!("tx disconnected");
        }
    }
}

// 马上试一次，有就返回
#[test]
fn test_try_recv() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        tx.send(String::from("hello world")).unwrap();
    });

    let result = rx.try_recv().unwrap();
    println!("Got: {}", result);
}

// 一次性取出通道中当前所有消息
// 适合在主循环里做定时清扫，比如每隔100ms 扫一遍是否有新消息，而不是等
#[test]
fn test_try_iter() {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        for _ in 0..10 {
            tx.send(String::from("hello world")).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::sleep(Duration::from_secs(5));
    for msg in rx.try_iter() {
        println!("Got: {}", msg);
    }
}

#[test]
fn test_multi_tx() {
    init_logger();
    let (tx, rx) = mpsc::channel::<String>();

    let tx2 = tx.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx.send(String::from("hello world from tx")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("hello world from tx2")).unwrap();
    });

    while let Ok(msg) = rx.recv() {
        info!("{}", format!("Got: {}", msg));
    }
}
