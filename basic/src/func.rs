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
