use std::cell::RefCell;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use std::thread;

/// 数据包装器
/// 本质是泛型容器，用来“修饰”或“控制”内部的数据的行为特性。
// Option<T>	表示可能有值也可能没有值	返回值/配置项/查找值
// Some(x)	Option<T> 的具体有值包装	Some(123) 表示有值
// Mutex<T>	提供线程安全的独占访问（加锁）	多线程计数器/全局状态共享
// OnceLock<T>	保证全局只初始化一次（线程安全）	配置文件、数据库连接池
// Arc<T>	多线程下的共享引用计数	在线程间共享数据
// Box<T>	把数据分配到堆上（heap）	递归结构/大数据
// RefCell<T>	运行时可变性（单线程）	某些结构内部需要动态修改
// Result<T, E>	表示可能成功也可能失败	错误处理

// 可选值	Option、Result
// 所有权/指针	Box、Rc、Arc
// 线程同步	Mutex、RwLock、OnceLock
// 可变性	RefCell、Cell
// 惰性初始化	Lazy、OnceLock

// Arc 是一个原子引用计数类型，它允许多个线程安全地共享对同一个数据的所有权。
// Arc 适用于在多线程环境下需要共享数据时，保证数据的安全性和有效性。
// 多线程共享：Arc 是线程安全的，可以在多个线程之间共享数据。
// 引用计数：Arc 会跟踪数据有多少个引用，一旦所有引用都被销毁，数据就会被释放。
// 不可变数据：Arc 本身是不可变的，也就是说，直接修改 Arc 中的数据是不允许的。如果需要修改数据，通常会结合 Mutex 或 RwLock 来实现。
// Arc 是为了解决多线程共享数据时的所有权管理问题，通常与 Mutex 一起使用来保证数据在多线程环境下的安全。
// 通过 Arc::clone 来增加引用计数，使得多个线程能够访问同一份数据。
// 如果数据不需要修改，可以直接使用 Arc 来进行不可变共享。

// Arc 只是一个 原子引用计数 的智能指针，它本身不会直接对数据进行锁定，也不会允许数据的修改。Arc 只是管理数据的所有权，并且能够在多个线程之间安全地共享数据。
// 它保证了数据在内存中存在，直到没有任何线程使用它时才会被释放，但它并不负责数据的并发修改。

#[test]
fn test_all_data_package() {
    let mut value = 10;
    let some_value = Some(value); // Option
    let mutex_value = Mutex::new(value); // Mutex
    let lock_value: OnceLock<i32> = OnceLock::new();
    lock_value.set(value).unwrap();
    let arc_value = Arc::new(value);
    let box_value = Box::new(value);
    let ref_cell = RefCell::new(Box::leak(box_value.clone()));

    // 取值
    println!("value is now {}", value);
    println!(
        "some_value is now {},value is {}",
        some_value.is_some(),
        some_value.unwrap()
    );
    if let Some(value) = some_value {
        println!("some_value is now {}", value);
    }
    println!("mutex_value is now {}", mutex_value.lock().unwrap());
    println!("lock_value is now {}", lock_value.get().unwrap());
    println!("arc_value is now {}", arc_value.clone());

    println!("box_value is now {}", box_value.clone());
    println!("ref_cell is now {}", ref_cell.borrow());

    println!("value is now {}", value);

    // 操作
    // mutex,多线程加锁操作,同一时间只有一个线程能读能改
    {
        let mut value = mutex_value.lock().unwrap();
        *value += 1;
    }
    println!(
        "mutex_value.lock is now {} , but value still is {}",
        mutex_value.lock().unwrap(),
        value
    );

    // Arc
    let arc_value = Arc::new(value);
    let mut handles = vec![];
    for _ in 0..5 {
        let arc_value = arc_value.clone();
        let handle = thread::spawn(move || {
            println!("in Arc<T> value is now {}", arc_value);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("arc value is {}", arc_value);

    // Arc<Mutex<T>>
    let arc_mutex_value = Arc::new(Mutex::new(value));
    let mut handles = vec![];
    for _ in 0..5 {
        let arc_mutex_value = arc_mutex_value.clone(); // 克隆 Arc，增加引用计数
        let handle = thread::spawn(move || {
            let mut num = arc_mutex_value.lock().unwrap(); // 读取 Arc 中的数据
            println!("in Arc<Mutext<T>> value is now {}", num);
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    } // Arc克隆计数为0
    println!("arc_mutex_value is now {}", arc_mutex_value.lock().unwrap());

    value += 1;
    println!("value is now {}", value);
    println!("lock_value is now {}", lock_value.get().unwrap()); // 基本类型已经解锁绑定，没有关联

    // 假如是复杂数据类型
    let mut my_str = String::from("My String");
    let my_lock_str = OnceLock::new();
    my_lock_str.set(my_str).unwrap();

    println!("my_lock_str is now {}", my_lock_str.get().unwrap());
    // my_str.push('!'); // value borrowed here after move
}

// 多读单写：多个线程可以同时读取数据（共享访问），但是如果有线程正在写数据（独占访问），其他线程就不能读取或写入。
// 写锁是排他的：在一个线程持有写锁时，其他线程既不能读取也不能写入数据。
// 读锁是共享的：多个线程可以同时持有读锁，允许多个线程同时读取数据，但不允许其他线程持有写锁。

// read()：获取一个读锁，多个线程可以同时读取数据，但在持有读锁时不能进行写操作。
// write()：获取一个写锁，写锁是排他的，只有一个线程可以进行写操作。

#[test]
fn test_rw_lock() {
    // 创建一个 RwLock，内部数据是一个整数
    let data = Arc::new(RwLock::new(10));

    let mut handles = vec![];

    for _ in 0..5 {
        let data_clone = Arc::clone(&data); // 克隆 Arc，增加引用计数
        let handle = thread::spawn(move || {
            let read_guard = data_clone.read().unwrap();
            println!("Read value is {}", read_guard);
        });
        handles.push(handle);
    }

    // 启动一个线程写操作
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut write_guard = data_clone.write().unwrap();
        *write_guard = 20;
        println!("Write value is {}", *write_guard);
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = data.read().unwrap();
    println!("final_value is {}", final_value);
}

#[derive(Debug)]
struct Counter {
    count: i32,
}
impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
    fn increment(&mut self) {
        self.count += 1;
    }
    fn get_count(&self) -> i32 {
        self.count
    }
}
#[test]
fn test_rw_lock_clock() {
    let counter = Arc::new(RwLock::new(Counter::new()));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let counter = counter_clone.read().unwrap();
            println!("Current count : {}", counter.get_count());
        });
        handles.push(handle);
    }

    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut counter = counter_clone.write().unwrap();
        counter.increment();
        println!("Current count : {}", counter.get_count());
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.read().unwrap();
    println!("final_count is {}", final_count.get_count());
}

/// Box

#[test]
fn test_box_use() {
    let mut b: Box<i32> = Box::new(5);
    println!("b = {}", b);
    *b += 10;
    println!("b = {}", b);

    // Box<str> 用来存储一个动态大小的字符串（str）。这种情况，Box 就是为了让我们能够将 str 存储到堆上
    let s: Box<str> = "hello world".into();
    println!("s = {}", s);
}

// RefCell

#[test]
fn test_ref_cell() {
    let value = RefCell::new(5);

    // 获取不可变借用
    {
        let mut v = value.borrow();
        // *v += 1; // cannot assign
        println!("value = {}", *v);
    }

    // 获取可变借用
    {
        let mut v = value.borrow_mut();
        *v += 10;
    }

    // 再次获取不可变借用
    {
        let v = value.borrow();
        println!("value = {}", *v);
    }
}

// dyn => dynamic; 实现多态，主要用于对参数限制，参数满足特定的dyn 后面跟着的trait类型就认为参数是合法的
// dyn 用于创建动态特征对象（trait objects），它允许在运行时确定具体的类型，而不是编译时静态确定。
// 通过 &dyn Trait 或 Box<dyn Trait> 等类型，Rust 可以在运行时动态调用实现了该特征的类型的方法。
// dyn 与 impl 区别：impl 进行静态分发，dyn 进行动态分发，后者允许运行时多态性，但会有一定的性能开销

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

// 传入的类型，满足了trait Speak 就行？
// greet 函数接受一个 dyn Speak 类型的参数，这意味着它可以接受任何实现了 Speak trait 的类型的引用。
fn greet(animal: &dyn Speak) {
    animal.speak();
}

#[test]
fn test_dyn() {
    let dog = Dog;
    let cat = Cat;

    greet(&dog);
    greet(&cat);

    let dog: Box<dyn Speak> = Box::new(dog);
    dog.speak();
}
