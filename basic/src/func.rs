// 1. 不能实现Copy特性的类型：
// String
// Vec<T>
// Box<T>
// Rc<T> / Arc<T>（智能指针）
// 文件句柄等

// 2.实现了Copy特性的标量类型
// 整数类型（包括符号与无符号）：
// i8, i16, i32, i64, i128, isize
// u8, u16, u32, u64, u128, usize
//
// 浮点数类型：
// f32, f64
//
// 布尔类型：
// bool （true 或 false）
//
// 字符类型：
// char （单个 Unicode 字符）

// ()（单元类型）
// 数组类型（如果其中的元素都是 Copy 类型）
// 元组（Tuple）如果元组中的所有元素都实现了 Copy，那么整个元组也可以实现 Copy 特性

use std::sync::mpsc::Receiver;

fn print_num(mut x: i32) {
    println!("x is {}", x);
    x += 10;
    println!("after add 10 in print_num func x is {}", x);
}

fn print_bool(mut x: bool) {
    println!("x is {}", x);
    x = false;
    println!("after add false in print_bool func x is {}", x);
}

// mut x: String：你传递了所有权，并且你可以在函数内对 x 进行修改，x 的所有权会转移到函数内，调用后原始变量会失效。
fn print_string(mut x: String) {
    println!("x is {}", x);
    x.push_str(" world");
    println!("after add string in print_string func x is {}", x);
}

// x: &mut String：你传递的是可变引用，函数内可以修改 x 的内容，但原始所有权不变，在函数调用后，x 仍然属于调用者。
// js默认的传递一个对象时是这一种
fn print_string_by_ref(x: &mut String) {
    println!("x is {}", x);
    x.push_str(" bye");
    println!("after add string in print_string_by_ref func x is {}", x);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

#[test]
fn test_closure() {
    // 闭包
    let scope_data = String::from("global_data");

    // 函数 (fn) 是在编译时确定的，它不支持动态捕获外部变量
    fn test() {
        println!("can't use scope_data");
        // Cannot capture a dynamic environment in a `fn` item
        // println!("scope data is {}", scope_data);
    }

    // 闭包 small_fn 可以捕获并访问外部的 scope_data 变量，因为闭包的设计允许它在运行时捕获其外部的环境（即外部变量）
    // 闭包在执行时会动态捕获外部作用域中的值，这使得闭包可以在运行时访问这些值，即使它们在闭包定义时并不在当前作用域内。
    // 闭包在值更改后，也能动态捕获最新的值执行
    let small_fn = || {
        println!("scope data is {}", scope_data);
    };
    small_fn();
}

#[test]
fn test_closure_mut() {
    let mut count = 0;
    let sum = || {
        println!("count is {}", count);
    };
    sum();
    // 外部不能改变闭包里的值，除非闭包里是（move）拷贝了一份外部的值
    // count += 1; // 执行会报错
    sum();
    println!("count is {}", count);
}

#[test]
fn test_closure_mut2() {
    let mut count = 0;
    // 如果sum里面有值是变化的，那闭包方法就必须是mut
    let mut sum = || {
        count += 1;
        println!("count is {}", count);
    };
    sum(); // count is 1
    sum(); // count is 2
    println!("count is {}", count); // count is 2
}

#[test]
fn test_closure_mut3() {
    let mut count = 0;

    let mut sum = move || {
        count += 1;
        println!("inner count is {}", count);
    };
    sum();
    sum();
    sum();

    // 当闭包使用了move，外部的count和闭包里的count已经没有关联了？
    println!("outside count is {}", count);
    count += 1;
    println!("outside count is {}", count);
}

#[test]
fn test_closure_ref() {
    let mut str = String::from("global_data");
    str.push_str(" world");

    // 闭包内部对外部变量的借用（可变或不可变）在闭包执行期间是持续有效的，直到闭包调用结束
    // 即最后一个change_str的调用
    // 在闭包执行期间，外部不能再对该变量进行任何其他形式的借用（包括修改或者读取），直到闭包执行结束并释放借用。
    let mut change_str = || {
        str.push_str(" 好");
        println!("str is {}", str);
    };

    change_str();
    change_str();

    // 后续代码，闭包change_str如果不再调用，闭包释放对str的借用
    str.push_str(" world");
    println!("str is {:?}", str);

    // 假如继续调用,那作用域内，应该保证str还是可变引用，且可变引用的掌控权在闭包内，那上面println!对str是不可变借用，则会报错
    // change_str();
}

#[test]
fn test_func() {
    // 普通类型是值的拷贝，即实现了Copy的类型
    let x = 10;
    print_num(x);
    println!("x is {}", x); // x依旧可用,且在print_num里不会改变x的值

    let b1 = false;
    print_bool(b1);

    // print_string拥有所有权str1
    let str1 = String::from("hello");
    print_string(str1);
    // 以下代码会报错，Value used after being moved，即str1已失效
    // println!("after run print_string func str1 is {:?}", str1);

    // print_string_by_ref借用str2
    let mut str2 = String::from("good");
    print_string_by_ref(&mut str2);
    println!("str2 is {}", str2);

    // 函数有返回值
    let result = sum(1, 2);
    println!("sum result is {}", result);
    println!("add i32 is {}", add(1, 2));
    println!("add f64 is {}", add(1.0, 2.1));
}

// Option<T>的场景：表示一个值可能存在也可能不存在的情况
/// Option 用来表示可能存在的值，适用于操作可能没有返回值的情况。
// Result 用来表示可能成功也可能失败的操作，适用于需要捕获和处理错误的场景
fn return_option(value: i32) -> Option<i32> {
    if value == 0 { None } else { Some(value) }
}
#[test]
fn test_result_option() {
    let result = return_option(10);
    match result {
        Some(n) => println!("result is {}", n),
        None => println!("result is None"),
    }
    assert_eq!(result, Some(10));

    let result = return_option(0);
    if let Some(result) = result {
        println!("result is {}", result);
    } else {
        println!("result is None");
    }

    let result = return_option(30);
    // 如果value是None，会panic
    let value = result.unwrap();
    println!("result is {}", value);

    let result = return_option(40);
    let doubled = result.map(|n| n * 2);
    println!("doubled {}", doubled.unwrap());

    // 获取Some里面的值 match, if let
}

// 验证返回Result
fn return_result(value: i32) -> Result<String, String> {
    if value > 0 {
        Ok("value is bigger than zero".to_string())
    } else {
        Err("value is less than zero".to_string())
    }
}

fn return_result_with_symbol() -> Result<String, String> {
    // 使用问号，更快捷的将错误传到外面去
    let value = return_result(0)?;
    println!("if success then run");
    Ok(value)
}

#[test]
fn test_result_result() {
    let result = return_result(0);
    match result {
        Ok(s) => println!("result is {}", s),
        Err(e) => println!("error is {}", e),
    }

    let result = return_result(30);
    if let Ok(value) = result {
        println!("result is {}", value);
    }

    let result = return_result(40);
    let value = result.unwrap();
    println!("result is {}", value);

    // 使用unwrap获取一个None值，会panic
    // let result = return_result(0);
    // let value = result.unwrap();
    // println!("result is {}", value);

    let result: Result<i32, String> = Ok(42);
    let value = result.map(|n| n * 2);
    println!("result is {}", value.unwrap());

    let result = return_result_with_symbol();
    match result {
        Ok(s) => println!("result is {}", s),
        Err(e) => println!("error is {}", e),
    }
}

fn might_fail(num: i32) -> Result<i32, &'static str> {
    if num > 0 {
        Ok(num)
    } else {
        // 这是因为你返回的是一个字符串引用（而不是 String），Rust 要知道这个引用的生命周期，否则它怕你引用了“悬垂”的内存。
        Err("Something went wrong")
    }
}

fn might_fail2(num: i32) -> Result<i32, String> {
    if num > 0 {
        Ok(num)
    } else {
        Err(String::from("Something went wrong"))
    }
}

#[test]
fn test_unwrap() {
    // Option的 unwrap使用
    let some_value = Some(5);
    let some_value_unwrap = some_value.unwrap();
    println!("some_value_unwrap is {}", some_value_unwrap);

    let some_value: Option<i32> = None;
    // panic
    // let some_value_unwrap = some_value.unwrap();
    // println!("some_value_unwrap will be panic {}", some_value_unwrap);
    match some_value {
        Some(n) => println!("some_value is {}", n),
        None => println!("some_value is None"),
    }

    if let Some(value) = some_value {
        println!("some_value is {:?}", value);
    } else {
        println!("some_value is None");
    }

    // 也会panic
    // let value = some_value.expect("some_value is None");
    // println!("some_value is {}", value);

    // Result 的unwrap
    let result = return_result(10);
    println!("{:?}", result);

    // 也会拿走所有权
    // match result {
    //     Ok(s) => println!("result is {}", s),
    //     Err(e) => println!("error is {}", e),
    // }

    let result_unwrap = result.unwrap(); // 也会拿走所有权
    // println!("{:?}", result.unwrap()); // unwrap会拿走所有权？
    // println!("{:?}", result); // 已经不能打印了，会panic

    let result = return_result(10);
    let result_ok = result.ok(); // 转换为Option类型
    println!("result_ok is {:?}", result_ok);

    let result = return_result(0);
    let result_ok = result.ok(); // 转换为Option类型
    println!("result_ok is {:?}", result_ok);

    let result = might_fail(-1);
    let value = result.ok(); // 不会报错，如果是错误，会为None
    println!("might_fail is {:?}", value);

    match result {
        Ok(value) => {
            println!("might_fail is {:?}", value);
        }
        Err(error) => {
            println!("might_fail is {:?}", error);
        }
    }
}

fn get_username(id: Option<&str>) -> Result<&str, String> {
    // 只有在 id 为 None 时才构造字符串，性能更好
    id.ok_or_else(|| "id is none".to_string())
}

fn get_username2(id: Option<&str>) -> Result<&str, String> {
    // 会始终构造 String - id is none（即使不需要用到）
    id.ok_or("id is none".to_string())
}

fn get_username3(id: Option<&str>) -> Result<&str, String> {
    let value = id.ok_or("id is none".to_string())?;
    Ok(value)
}

// 测试option和result之间互相转化
#[test]
fn test_option_to_result() {
    let opt: Option<i32> = Some(10);
    let opt2: Option<i32> = None;
    println!("opt is {:?} opt2 is {:?}", opt, opt2);
    println!("opt ok is {:?}", opt.ok_or(1));

    // Option转Result Option<T> -> Result<T,E>
    // ok_or :直接提供错误值（立即创建）
    // ok_or_else(|| err): 延迟构造错误值，更高效。
    // match手动转换: 更灵活，适合复杂逻辑。
    // map_or_else: 更通用：可以在 None 和 Some 时分别处理，返回 Result 也是合法的
    // ？（只能在返回Result的函数中）: ? 自动帮你做 None => Err(...) 的转换
    let opt2_ok_or = opt2.ok_or("opt2 is None");
    println!("opt2_ok_or is {:?}", opt2_ok_or);
    match opt2_ok_or {
        Ok(n) => println!("opt2_ok_or result is {}", n),
        Err(e) => println!("opt2_ok_or error is {}", e),
    }
    if let Err(_) = opt2_ok_or {
        println!("opt2_ok_or is Err");
    }

    // 当你的错误值是懒计算的（比如一个函数调用，或者构造代价较大的错误），推荐用 ok_or_else
    // 只有在 None 时才会调用闭包里的逻辑，避免不必要的计算
    let opt2_ok_or_else: Result<i32, String> = opt2.ok_or_else(|| {
        println!("opt2_ok_or_else is Err in closure");
        "error from closure".to_string()
    });
    println!("opt2_ok_or_else is {:?}", opt2_ok_or_else);

    let result1 = get_username(None);
    let result2 = get_username2(None);
    println!("result1 is {:?} result2 is {:?}", result1, result2);

    let res = match Some(10) {
        Some(n) => Ok(n),
        None => Err("no data"),
    };
    println!("res is {:?}", res);

    let opt = Some("hello");
    let result = opt.map_or_else(|| Err("空值"), |v| Ok(v.to_uppercase()));
    println!("result is {:?}", result);

    let opt = None;
    let result = opt.map_or_else(|| Err("空值"), |v: &str| Ok(v.to_uppercase()));
    println!("result is {:?}", result);

    let result = get_username3(None);
    println!("result is {:?} ", result);
}

fn get_result(value: i32) -> Result<i32, &'static str> {
    if value > 0 {
        Ok(value)
    } else {
        Err("value is less than zero")
    }
}
#[test]
fn test_result_to_option() {
    // ok
    // err
    // map_or
    // map_or_else
    // ok().flatten
    // match

    // 使用OK: 丢失错误信息，只关心是否有值，不需要处理错误的情况
    let result = get_result(10);
    let opt = result.ok();
    println!("opt is {:?}", opt); // Some(10)

    let result = get_result(-1);
    let opt = result.ok();
    println!("opt is {:?}", opt); // None

    // err：丢失成功结果，调试、记录错误用
    let result = get_result(0);
    let opt = result.err();
    println!("result is {:?}", opt); // Some("value is less than zero")

    let result = get_result(1);
    let opt = result.err();
    println!("result is {:?}", opt); // None

    // ok().flatten(): 专门用于 Result<Option<T>, E> 场景,仅适用于嵌套结构,一步拿到最终值，简洁

    // map_or(default,闭包callback): 提供默认值，避免 Option 的处理,无法获得错误或上下文,有明确默认值的场景
    // 等于match错误返回default，成功执行callback
    let result = get_result(0);
    let opt = result.map_or(123, |v| v * 2);
    println!("opt is {:?}", opt);

    let result = get_result(10);
    let opt = result.map_or(123, |v| v * 2);
    println!("opt is {:?}", opt);
    // 等同于
    let opt = match result {
        Ok(n) => n * 2,
        Err(_) => 123,
    };
    println!("opt is {:?}", opt);

    // map_or_else：懒加载性能更优
    let opt = result.map_or_else(|v| 123, |v| v * 2);
    println!("map_or_else opt is {:?}", opt);

    let result = get_result(0);
    let opt = result.map_or_else(|v| 123, |v| v * 2);
    println!("map_or_else error opt is {:?}", opt);
}
