use std::thread;
use std::time::Duration;

fn workout(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!("Today, do you want to ignore this check?");
        action();
    } else if random_number == 3 {
        println!("xxxxx");
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher { query, value: None }
    }

    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                println!("不存在缓存，重新计算获取,args is {}", args);
                let v = (self.query)(args);
                self.value = Some(v);
                v
            }
        }
    }
}

/// 闭包捕获变量有三种途径，对应函数参数的三种传入方式：转移所有权，可变借用，不可变借用
/// Fn(不可变借用）
/// FnMut(可变借用）
/// FnOnce（获取所有权）

// Fn不可变借用
fn call_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// 可变借用
fn call_fn_mut<F: FnMut()>(mut f: F) {
    f()
}

// fnOnce
fn call_fn_once<F:FnOnce()>(f: F) {
    f();
}

#[cfg(test)]
mod tests {
    use crate::closure::{Cacher, call_fn, call_fn_mut, workout, call_fn_once};

    #[test]
    fn test_no_closure() {
        // 测试非闭包
        let x = 1;

        // 以下代码会直接报错，test_func里拿不到x的值
        // fn test_func(y: i32) -> i32 {
        //     x + y
        // }
        // let result = test_func(2);
        // assert_eq!(result, 3);

        // 使用闭包就可以
        let test_func = |y| x + y;
        assert_eq!(test_func(2), 3);
    }

    #[test]
    fn test_closure() {
        let mut x = 1;
        let sum = |y| x + y;
        assert_eq!(sum(2), 3);
        assert_eq!(x, 1);
        assert_eq!(sum(3), 4);

        // 以下代码会报错：
        // 预期在x被捕获为不可变的值后，重新对x赋值（可变）时程序崩溃
        // x = 20;
        // assert_eq!(sum(10), 30);

        // 但可以在更改x之后，重新定义sum，调用这个新的sum方法
        x = 20;
        let sum = |y| x + y;
        assert_eq!(sum(10), 30);
        assert_eq!(sum(10), 30);

        // 或者使用move将x所有权转移
        // 而后更改x的值，不影响sum方法里的x的值
        x = 50;
        let sum = move |y| x + y;
        assert_eq!(sum(10), 60);
        x = 60;
        assert_eq!(sum(10), 60);
        assert_eq!(x, 60);

        // 在闭包里打印
        let sum = move |y| {
            println!("current x is {}", x);
            x + y
        };
        assert_eq!(sum(10), 70);
        x = 80;
        assert_eq!(sum(10), 70);
        assert_eq!(x, 80);
    }

    #[test]
    fn test_three_closure() {
        // 1.不可变借用
        let num = 10;
        let add = |x| num + x;
        assert_eq!(call_fn(add, 5), 15);
        assert_eq!(call_fn(add, 10), 20);

        // 2.可变借用
        let mut count = 0;
        #[allow(unused)]
        let increment = || count += 1;
        // 因为对count进行了操作，所以以下执行报错：consider changing this to be mutable
        // increment();
        let mut increment = || count += 1;
        call_fn_mut(&mut increment);
        call_fn_mut(&mut increment);
        increment();
        assert_eq!(count, 3);

        // `count` is assigned to here but it was already borrowed
        // 所以以下代码，给count重新赋值（不会报错）后，调用increment会报错
        count = 10;
        assert_eq!(count, 10);
        // increment();
        // assert_eq!(count, 11);

        // 3.获取所有权
        let mut current = 1;
        let consume = move || println!("current is {}", current);
        consume();
        current = 10;
        assert_eq!(current, 10);
        consume(); // 但这里执行还是打印出的1

        call_fn_once(consume);
    }

    #[test]
    fn test_workout() {
        workout(30, 3);
    }

    #[test]
    fn test_cacher() {
        let mut cacher = Cacher::new(|x| x + 1);
        assert_eq!(cacher.value(1), 2);
        assert_eq!(cacher.value(1), 2);

        // self.value已经缓存了计算的值，传入新的值也并不会重新计算和返回
        assert_ne!(cacher.value(3), 4);
    }
}
