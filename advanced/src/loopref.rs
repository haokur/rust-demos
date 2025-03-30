use std::cell::{Ref, RefCell};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::{Rc, Weak};

/// 循环引用

#[test]
fn test_weak() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    println!("five strong_count is {}", Rc::strong_count(&five));
    println!("weak_count is {}", Rc::weak_count(&five));

    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);

    drop(five);
    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);
}

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

#[test]
fn test_gadget() {
    let gadget_owner = Rc::new(Owner {
        name: "jack".to_string(),
        gadgets: RefCell::new(vec![]),
    });

    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    });
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    });

    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget1));
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget2));

    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[test]
fn test_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {},weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        println!(
            "branch strong = {},weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {},weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {},weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {},weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

/// 结构体自引用
#[derive(Debug)]
struct SelfRef<'a> {
    value: String,

    point_to_value: &'a str,
}

#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

impl<'a> WhatAboutThis<'a> {
    fn tie_the_knot(&'a mut self) {
        self.nickname = Some(&self.name[..4]);
    }
}

#[test]
fn test_struct_ref() {
    let s = "aaa".to_string();
    // 如下直接使用，编译报错
    // let v = SelfRef {
    //     value: s,
    //     point_to_value: &s,
    // };

    let mut tricky = WhatAboutThis {
        name: "annnablle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name);
    println!("{:?}", tricky);

    let mut tricky2 = WhatAboutThis {
        name: "annnablle".to_string(),
        nickname: None,
    };
    tricky2.tie_the_knot();

    // 提示报错：cannot borrow `tricky2` as immutable because it is also borrowed as mutable
    // println!("{:?}", tricky2);
}

#[derive(Debug)]
struct Unmovalbe {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovalbe {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovalbe {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.data);
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

#[test]
fn test_pin() {
    let unmoved = Unmovalbe::new("hello".to_string());
    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
}
