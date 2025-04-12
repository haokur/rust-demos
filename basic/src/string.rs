// 这里返回的已经和传入的s解耦
fn first_word(s: &str) -> Option<String> {
    if let Some(first_char) = s.chars().next() {
        Some(first_char.to_string())
    } else {
        None
    }
}

// 返回指针问题在于，对应的s不能改变了
// 因为假如s改变，那first_word2里引用的内容需要发生变化
// 而这是预期之外的变化，如果s被清理，那返回的则可能是悬垂指针
fn first_word2(s: &str) -> &str {
    &s[0..1]
}

#[test]
fn test_borrowed_str() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("word is {:?}", word);
    s.clear();
    println!("word is {:?}", word);
}

#[test]
fn test_borrowed_str2() {
    let mut s = String::from("hello world");
    let word = first_word2(&s);
    println!("word is {:?}", word);
    // s.clear();
    // println!("word is {:?}", word);
}

#[test]
fn test_borrowed_str3() {
    // 字符串字面量是切片
    let s = "hello world";
    let word = first_word2(s);
    println!("word is {:?}", word);

    let str = String::from("hello world");
    // 使用as_str，将String转换为&str
    let str = str.as_str();
    println!("str is {:?}", str);
}

#[test]
fn test_change_str() {
    let mut str = String::from("hello");
    // 添加字符串
    str.push_str(" world");
    // 添加char
    str.push('!');
    println!("str is {:?}", str);

    // 指定位置插入
    str.insert(5, ',');
    println!("str is {:?}", str);

    str.insert_str(6, "rust");
    println!("str is {:?}", str);

    // 替换
    str = str.replace("world!", "program");
    println!("str is {:?}", str);

    let mut str = String::from("good good study,day day up");
    // 可控制数量的替换
    str = str.replacen("good", "nice", 1);
    println!("str is {:?}", str);

    str = str.replacen("day", "everyday", 2);
    println!("str is {:?}", str);

    // 索引区间替换,没有返回值，在原字符串上更改
    str.replace_range(0..4,"good");
    println!("str is {:?}", str);
}
