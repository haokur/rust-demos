use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

// 静态常量
const MAX_ID: usize = usize::MAX / 2;
// 静态变量
static REQUEST_RECV: usize = 0;

#[test]
fn test_global_var() {
    println!("global_var MAX_ID = {}", MAX_ID);

    println!("global_var REQUEST_RECV = {:?}", REQUEST_RECV);
}

struct Factory {
    factory_id: usize,
}
static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID2: usize = usize::MAX / 2;

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID2 {
        println!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID2 {
        println!("Factory ids overflowed");
    }
    next_id
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}

#[test]
fn test_generate_global_id() {
    let factory = Factory::new();
    println!("factory.factory_id = {}", factory.factory_id);

    let factory = Factory::new();
    println!("factory.factory_id = {}", factory.factory_id);
}

// 运行期初始化时报错
// cannot call non-const associated function `<String as From<&str>>::from` in statics
// static NAMES: Mutex<String> = Mutex::new(String::from("Hello"));

// 使用lazy_static实现
lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Hello"));
}

#[test]
fn test_init_value() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(" world");
    println!("{}", v);
}

lazy_static! {
    static ref UserData: HashMap<u32, &'static str> = {
        println!("执行初始化");
        let mut m = HashMap::new();
        m.insert(0, "World");
        m.insert(1, "UserData");
        m
    };
}

#[test]
fn test_global_user_data() {
    println!("{}", UserData.get(&0).unwrap_or(&""));
    println!("{}", UserData.get(&1).unwrap_or(&""));
}

#[derive(Debug)]
#[allow(unused)]
struct Config {
    a: String,
    b: String,
}

#[allow(unused)]
static mut CONFIG: Option<&mut Config> = None;

#[test]
fn test_leak_to_global() {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    println!("c is {:?}", c);

    // unsafe {
    //     CONFIG = Some(Box::leak(c));
    //     println!("CONFIG = {:?}", &CONFIG);
    // }
}

/// 错误处理

#[test]
fn test_error() {
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let e1: Result<&str, &str> = Err("err1");
    let e2: Result<&str, &str> = Err("err2");

    assert_eq!(s1.or(s2), s1);
    assert_eq!(s1.or(n), s1);

    assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
    assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
    assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
    assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

    assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
    assert_eq!(s1.and(n), n);   // Some and None = None
    assert_eq!(n.and(s1), n);   // None and Some = None
    assert_eq!(n.and(n), n);    // None1 and None2 = None1

    assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
    assert_eq!(o1.and(e1), e1); // Ok and Err = Err
    assert_eq!(e1.and(o1), e1); // Err and Ok = Err
    assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
}
