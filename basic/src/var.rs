#[test]
fn test_var() {
    let x = 10;
    let y = 3.14;

    let a = String::from("hello");
    let b = "world";

    let mut c = 20;
    println!("origin c is {}", c);
    c = 30;
    println!("re_set c is {}", c);

    let c = 40;
    println!("redefined c is {}", c);

    let arr = vec![1, 2, 3];
    let arr2 = vec!["a", "b", "c"];

    let tup = (1, "hello");
    let (e, f) = tup;

    println!("{x},{y},{a},{b},{c},{:?},{:?},{:?} {e},{f}", arr, arr2, tup);

    let g = false;
    let h = true;
    let i = ();

    println!("{g},{h},{:?}", i);

    // 不支持mut，即不支持更改
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS);
}
