// ❌这样写会报错，get_str执行完，hello_str就会失效，返回的&hello_str则会报错
// fn get_str<'a>() -> &'a str {
//     let hello_str = String::from("Hello, World!");
//     &hello_str
// }

// >>> ❌使用static生命周期，即整个程序生命周期内都有效的字符串
// fn get_str() -> &'static str {
//     let hello_str = String::from("Hello, World!");
//     &hello_str
// }

// >>> 上面写法同样报错，
// 但你却返回了一个生命周期极短（只活在函数内部）的引用，这是说谎，Rust 编译器当然不允许你这么干
// ✅使用字面量
fn get_str() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_lifecycle() {
    let get_hello_str = get_str();
    println!("Got hello str: {:?}", get_hello_str);
}

// ❌Missing lifetime specifier
// fn longest(s1: &str, s2: &str) -> &str {
//     if s1.len() > s2.len() { s1 } else { s2 }
// }

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

#[test]
fn test_longest() {
    let a = String::from("Hello");
    let b = String::from("World");

    let res = longest(&a, &b);
    println!("res: {:?}", res);
}

// 结构体
struct Person<'a> {
    name: &'a str,
}

#[test]
fn test_struct() {
    let name = String::from("John");
    let p = Person { name: &name };

    println!("Person name: {:?}", p.name);
}

// 返回值由使用时确定
trait MagicResult {
    fn magic() -> Self;
}

impl MagicResult for String {
    fn magic() -> Self {
        "hello".to_string()
    }
}

impl MagicResult for &str {
    fn magic() -> Self {
        "hello"
    }
}

impl MagicResult for u8 {
    fn magic() -> Self {
        10
    }
}

fn func<T: MagicResult>() -> T {
    T::magic()
}

#[test]
fn test_multi_type_result() {
    let a: String = func();
    let b: u8 = func();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}
