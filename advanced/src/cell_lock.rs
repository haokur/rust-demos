// OnceLock<T> 是 Rust 标准库中提供的一个线程安全、只能初始化一次的容器，
// 用来实现 延迟初始化（lazy initialization）+ 单例模式（singleton） 的场景。
// 一个可以全局使用、只能被 set() 一次、之后只能 get() 读取的变量。

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

static CONFIG: OnceLock<String> = OnceLock::new();

lazy_static! {
    static ref LAZY_CONFIG: Mutex<String> = Mutex::new("production".to_string());
}

fn test_once_lock_value(value: &String) {
    println!("test_once_lock_value {}", value);
}

#[test]
fn test_once_lock() {
    if let Some(value) = CONFIG.get() {
        println!("test get after init,value is {}", value);
    } else {
        println!("未被初始化");
    }

    CONFIG
        .set("production".to_string())
        .expect("only can be set once!!!");

    // CONFIG
    //     .set("production".to_string())
    //     .expect("报错只能被设置一次");

    let value = CONFIG.get().unwrap();
    println!("value: {}", value);

    test_once_lock_value(value);
    println!("value: {}", CONFIG.get().unwrap());
}

// lazy_static! 与OnceLock效果基本一致
#[test]
fn test_lazy_static() {
    println!("test_lazy_static value is {}", LAZY_CONFIG.lock().unwrap());

    test_once_lock_value(&LAZY_CONFIG.lock().unwrap());

    println!("test_lazy_static value is {}", LAZY_CONFIG.lock().unwrap());
}

#[test]
fn test_mutex() {
    let counter = Mutex::new(0);

    {
        // .lock() 会“上锁”以访问里面的数据，保证当前只有这一个线程可以访问和修改它
        let mut num = counter.lock().unwrap(); // 加锁
        *num += 1; // 修改
    } // 自动释放锁

    println!("counter is {:?}", counter);
    println!("counter is {}", counter.lock().unwrap());
}

#[test]
fn test_mutex2() {
    let counter = Mutex::new(0);
    let counter = Arc::new(counter);

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("num is {}", num);
            *num += 2;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter is {:?}", counter);
}
