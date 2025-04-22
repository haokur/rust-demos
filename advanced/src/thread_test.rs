use crate::helpers;
use log::info;
use std::io::Write;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use std::time::Duration;
use std::{io, thread};

#[test]
fn test_basic_thread() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("thread running");
    });

    println!("main running");
    handle.join().unwrap(); // 等待handle线程执行结束，再往下执行
    println!("main running after wait thread all run finished");
}

#[test]
fn test_multi_thread() {
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            println!("thread running {}", i);
        });
        handles.push(handle);
    }
    println!("main running");
    // 等待上面所有线程执行完
    for handle in handles {
        handle.join().unwrap();
    }
    println!("main running after wait thread all run finished");
}

// 线程间共享数据考虑两个问题：
// 1.所有权问题：Rust 不允许多个线程同时拥有一个数据的可变引用（借用检查器限制）
// 2.同步问题：多线程并发访问共享数据时，必须同步访问，避免数据竞争（data race）
// - Arc<T>，原子引用计数，用于在多个线程间共享所有权（Thread-safe 的 Rc）
// - Mutex<T>，互斥锁，确保某一时刻只有一个线程能访问数据
// - RwLock<T>，读写锁，允许多个读，但写时只能有一个写者

// 使用 Arc<Mutex<T>>
// 场景：同一时间只有一个能读能写
#[test]
fn test_thread_arc_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("结果：{}", *counter.lock().unwrap());
}

// Arc<RwLock<T>>
// 同一时间可多个读，有一个写的时候，其他线程的读写都停止等待这个写完
// 当有一个写线程加锁时，会阻塞所有读/写线程，直到它释放
// 场景：读多写少

#[test]
fn test_thread_arc_rwlock() {
    let data = Arc::new(RwLock::new(vec!["初始化数据".to_string()]));

    let mut handles = vec![];

    // 启动多个读线程
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for _ in 0..=3 {
                let r = data.read().unwrap();
                println!("[读取线程{i}] 数据内容:{:?}", *r);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    // 一个写线程，写数据
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut w = data.write().unwrap();
            println!("[写线程]正在写入...");
            thread::sleep(Duration::from_millis(100));
            w.push("写入数据".to_string());
            println!("[写线程]写入完成...");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_mpsc_rx_thread() {
    helpers::logger::init_logger();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            info!("{}", format!("[监听线程] 收到消息:{}", msg));
        }
    });

    for i in 0..3 {
        tx.send(format!("消息: {}", i)).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }

    // 模拟结束
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(5000));
    });
    handle.join().unwrap();

    info!("主线程发送完毕");
}


