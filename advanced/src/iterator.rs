use std::collections::HashMap;

/// 迭代器适配器：enumerate，zip，filter，map
/// 消费适配器：collect，sum, fold
#[test]
fn test_for() {
    let arr = [1, 2, 3];
    for v in arr {
        print!("{};", v);
    }
    println!("\n------");

    for i in 1..10 {
        print!("{};", i);
    }
    println!("\n------");

    // 左闭右闭
    for i in 1..=10 {
        print!("{};", i);
    }
    println!("\n------");

    let v = vec![2, 3, 4, 5, 6];
    for (i, v) in v.iter().enumerate() {
        println!("第{}个值是{}", i, v);
    }
}

#[test]
fn test_iterator() {
    let arr = [1, 2, 3];
    for v in arr.iter() {
        print!("{};", v);
    }
    println!("\n------");

    let mut arr_iter = arr.iter();
    assert_eq!(arr_iter.next(), Some(&1));
    assert_eq!(arr_iter.next(), Some(&2));
    assert_eq!(arr_iter.next(), Some(&3));
    assert_eq!(arr_iter.next(), None);

    let arr_vec = vec![1, 2, 3];
    let mut arr_iter = arr_vec.iter();
    assert_eq!(arr_iter.next(), Some(&1));
}

/// into_iter 夺走所有权
/// iter 借用
/// iter_mut 可变借用
#[test]
fn test_iterator_three_api() {
    let values = vec![1, 2, 3];

    // into_iter 夺走所有权, v是 i32
    for v in values.into_iter() {
        print!("{};", v);
    }
    // 以下会报错，因为values已经在values.into_iter时被夺走所有权
    // println!("value is {:?}", values);

    println!("\n------");

    // iter 借用, v是 &i32
    let values = vec![1, 2, 3];
    for v in values.iter() {
        print!("{};", v);
    }
    println!("values is {:?}", values);

    // iter_mut 可变借用, v是 &mut i32
    let mut values = vec![1, 2, 3];
    for v in values.iter_mut() {
        *v *= 2;
    }
    println!("values is {:?}", values);
}

#[test]
fn test_iterator_consume() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    println!("{:?}", v1);
    // Error: Value used after being moved
    // println!("{:?}", v1_iter);

    // collect是一个消费适配器，使用它将一个迭代器中的元素收集到指定类型中
    let v2 = vec![1, 2, 3];
    let results: Vec<i32> = v2
        .iter()
        .map(|x| {
            println!("x is {};", x);
            x + 1
        })
        .collect();
    println!("after map add one result is {:?}", results);
}

// zip是一个迭代器适配器，将两个迭代器的内容按顺序索引压缩在一起
#[test]
fn test_zip_collect() {
    let names = ["jack", "bob"];
    let ages = [18, 25];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("folks result is {:?}", folks);

    let names = ["jack", "bob"];
    let ages = [18, 25];
    let zip_result = names.into_iter().zip(ages.iter());
    let zip_result_clone = zip_result.clone();
    for x in zip_result {
        println!("zip_result item is {:?}", x);
    }

    let zip_collect_result: HashMap<_, _> = zip_result_clone.collect();
    println!("zip_collect result is {:?}", zip_collect_result);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_custom_iterator() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // zip,map,filter是迭代适配器，sum是消费者适配器
    // zip => [(1,2),(2,3),(3,4),(4,5)]
    // map => [2,6,12,20]
    // filter => [6,12]
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}

#[test]
fn test_enumerate() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let val = v
        .iter()
        .enumerate()
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        .fold(0, |sum, acm| sum + acm);
    println!("fold result is {:?}", val);
}
