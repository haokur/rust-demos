#[test]
fn test_type() {
    let value = Some(10);

    let is_some = value.is_some();
    assert_eq!(is_some, true);

    let data = value.unwrap();
    let data2 = value.expect("获取值失败");

    println!("{:?} {} {}", value, data, data2);

    if let Some(data3) = value {
        println!("data3 is {}", data3);
    }
    match value {
        Some(data4) => {
            println!("data2 is {}", data4)
        }
        None => {
            println!("data2 is None")
        }
    }
    let value = Some(10);
    let data5 = if value.is_some() { value.unwrap() } else { 0 };
    println!("data5 is {}", data5);

    let data6 = value.map(|x| x * 2);
    println!("data6 is {:?}", data6);

    let data7 = value.ok_or("获取失败");
    println!("data7 is {:?}", data7);
    let data8 = value.ok_or_else(|| "获取失败");
    println!("data8 is {:?}", data8);

    // value 和 and 里面的都有值时=》取and参数值，否则都是None
    let data9 = value.and(Some(99));
    println!("data9 is {:?}", data9);

    let value: Option<i32> = None;
    let data10 = value.and(Some(199));
    println!("data10 is {:?}", data10);

    let value = Some(999);
    let data11 = value.and_then(|x| if x > 100 { Some(x) } else { None });
    println!("data11 is {:?}", data11);

    let value2: Option<i32> = None;
    let data12 = value2.unwrap_or_else(|| 88);
    println!("data12 is {:?}", data12);

    // println!("value: {:?}", value);
    //
    // let value = None;
    // let is_none = value.is_none();
    // assert_eq!(is_none, true);
}
