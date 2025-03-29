use std::convert::TryInto;
use std::fmt;
use std::ops::Add;

#[test]
fn test_type_transform() {
    println!("i32 max {}", i32::MAX);

    let a = 3.1 as i8;
    println!("a is {}", a);

    // 内存地址转换为指针
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    println!("p1 is {:?}", p1);

    let first_address = p1 as usize; // 将p1的内存地址转换为一个整数
    println!("first_address is {:?}", first_address);
    let second_address = first_address + 4; // 访问该地址指向的下一个整数
    println!("second_address is {:?}", second_address);

    // p2指向的是values的第二个元素
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
    println!("{:?}", &p2);
    println!("{:?}", *&p2);
}

#[test]
fn test_try_into() {
    let a: u8 = 10;
    let b: u16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    if a < b_ {
        println!("a is {:?}", a);
    }
}

struct Foo {
    x: u32,
    y: u16,
}

#[derive(Debug)]
struct Bar {
    a: u32,
    b: u16,
}
fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

#[test]
fn test_struct_transform() {
    let foo = Foo { x: 1, y: 2 };
    let bar = reinterpret(foo);
    println!("bar is {:?} bar.a is {},bar.b is {}", bar, bar.a, bar.b);
}

/// newtype

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}
#[test]
fn test_new_type() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w is {}", w);
}

struct Meters(u32);
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "目标地点距离你{}米", self.0)
    }
}
impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
}

#[test]
fn test_custom_add_display() {
    let d = calculate_distance(Meters(10), Meters(20));
    println!("distance is {}", d);
}

/// 类型别名
type OtherType = u32;

#[test]
#[allow(unused)]
fn test_alias_type() {
    let x: u32 = 5;
    let y: OtherType = 20;
    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    fn return_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type2(f: Thunk) {}
}
