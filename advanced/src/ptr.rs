use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

/// 智能指针

fn foo(x: &str) -> String {
    let a = "hello,".to_string() + x;
    a
}
#[test]
fn test_ptr() {
    let b = foo("rust");
    println!("{}", b);
}

// 使用Box<>
#[test]
fn test_box_heap() {
    let a = Box::new(3);
    println!("a = {}", a);
    // let b = a + 1; // Cannot add `i32` to `Box<i32>`

    let b = *a + 1;
    println!("b = {}", b);
}

/// 避免栈上数据的拷贝,使用Box可以将原来放在栈中的数据强制放在堆上
#[test]
fn test_skip_copy() {
    let arr = [0; 100];
    let arr1 = arr;
    // 因为arr分配在栈上，所以arr1是拷贝arr，所以arr能继续用
    println!("arr = {:?}", arr);
    println!("arr1 = {:?}", arr1);

    // 下面s1赋值给s2后。s1已经不能使用了
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 = {}", s1); // Value used after being moved
    println!("s2 = {}", s2);

    // 在堆上创建数组，然后用智能指针指向
    let arr = Box::new([0; 100]);
    let arr1 = arr;
    // println!("arr1 = {:?}", arr); // Value used after being moved
    println!("arr1 = {:?}", arr1);
}

#[allow(unused)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// 特征对象
trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id);
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这是屏幕上第{}个选择框", self.id);
    }
}

#[test]
fn test_box_trait() {
    // let elems = vec![Button { id: 1 }, Select { id: 2 }]; // 类型不一致不能放在一个数组内

    // 以下代码报错：the trait `Sized` is not implemented for `dyn Draw`
    // let elems: Vec<dyn Draw> = vec![Button { id: 1 }, Select { id: 2 }];
    // for e in elems {
    //     e.draw();
    // }

    // 定义elems的类型为共同实现了draw这个trait的struct
    // 特征对象指的是：Box<dyn Draw>
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    for e in elems {
        e.draw();
    }
}

#[test]
fn test_box_use() {
    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    // 第一个*是取出box指针，第二个*是取出box指针对应的值
    // step1: &Box<i32> => Box<i32>
    // step2: Box<i32> => i32
    let sum = **first + **second;
    println!("sum = {}", sum);

    // let (first, second) = (arr[0], arr[1]); // 直接报错，can't move
}

fn gen_static_str() -> &'static str {
    let mut s = String::from("hello");
    s.push_str(", world!");
    // 使用leak将一个运行期的值转为'static
    // 应用场景是你需要一个运行期初始化的值，但是可以全局有效（和整个程序生命周期一致）
    Box::leak(s.into_boxed_str()) as &str
}

#[test]
fn test_box_leak() {
    let s = gen_static_str();
    println!("s = {}", s);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn display(self: &mut Person) {
        let Person { name, age } = &self;
        println!("name = {}, age = {}", name, age);
    }
}

#[test]
fn test_origin() {
    let mut p1 = Person {
        name: String::from("jack"),
        age: 18,
    };
    p1.display();

    let mut p1 = Person::new("jack".to_string(), 18);
    p1.display();
}

fn display_string(s: &str) {
    // 这里s为引用，但是会自动解引用，打印出s的真正的值？
    println!("string = {}", s);
}
#[test]
fn test_deref() {
    let x = 5;
    let y = &x;
    println!("x = {}", x);
    println!("y = {}", y);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Box
    let x = Box::new(5);
    let sum = *x + 1;
    println!("sum = {}", sum);

    let s = String::from("hello");
    display_string(&s);

    // 多层自动解引用
    let s = Box::new(String::from("hello"));
    // 编译期间：rust会分析并连续使用Deref直到最终获取到一个引用来匹配函数或方法的参数类型
    display_string(&s);

    let s1: &str = &s;
    let s2: String = s.to_string();
    println!("s1 = {}, s2 = {}", s1, s2);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_custom_box() {
    let x = MyBox::new(5);
    assert_eq!(5, *x);
    // println!("x.deref() = {}", x.deref());
    // assert_ne!(5, x.deref());
    assert_eq!(5, *(x.deref()));
}

/// 引用计数，Rc，Arc
#[test]
fn test_rc() {
    let s = String::from("hello");
    let a = Box::new(s);
    // println!("s={}", s); // Value used after being moved
    // let b = Box::new(s); // Value used after being moved
    println!("a={}", a);

    // 使用Rc解决
    let a = Rc::new(String::from("hello"));
    // 仅clone了智能指针和增加了引用计算，并未克隆底层数据
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    println!("a = {}", a);

    // 'a' cannot borrow as mutable，即使给a加上mut。因为rc是不可变引用
    // a.push_str("world");
    // println!("a = {}", a);
    {
        let c = Rc::clone(&a);
        println!("c = {}", c);
        assert_eq!(3, Rc::strong_count(&a));
    }
    // 当离开c的作用域，那么引用计算会减去对应c作用域增加的数
    assert_eq!(2, Rc::strong_count(&a));
}

struct Owner {
    name: String,
}
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

#[test]
fn test_rc_example() {
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "jack".to_string(),
    });

    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };

    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    println!("gadget_owner count is {}", Rc::strong_count(&gadget_owner));
    drop(gadget_owner);
    println!(
        "gadget_owner count is {} after drop",
        Rc::strong_count(&gadget1.owner)
    );
    // 释放掉gadget_owner，但是下面可以接着使用？
    // drop掉的只是gadget_owner的一个引用
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
}

/// Arc Atomic Rc，原子化的Rc<T> 智能指针，原子化是一种并发原语
/// 保证数据能够安全地在线程间共享

#[test]
fn test_arc_example() {
    let s = Arc::new(String::from("hello"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        #[allow(unused)]
        let handle = thread::spawn(move || {
            println!("s = {}", s);
        });
    }
}

#[test]
fn test_cell() {
    let c = Cell::new("hello");
    let one = c.get();
    println!("one = {}", one);
    c.set("world");
    let two = c.get();
    println!("one = {},two = {}", one, two);
}

#[test]
fn test_ref_cell(){
    let s = RefCell::new(String::from("hello"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();
    println!("s1 = {}, s2 = {}", s1, s2);
}
