pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Welcome {}!", name)
}

pub fn fibonacci_u64(number: u64) -> u64 {
    let mut last: u64 = 1;
    let mut current: u64 = 0;
    let mut buffer: u64;
    let mut position: u64 = 1;

    return loop {
        if position == number {
            break current;
        }

        buffer = last;
        last = current;
        current = buffer + current;
        position += 1;
    };
}

pub struct Guess {
    #[allow(unused)]
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        if value < 1 {
            panic!("Guess value must be greater than or equal to 1");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100");
        }

        Guess { value }
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// #[cfg(test] 标注只有cargo test时才编译和运行tests模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    #[ignore]
    #[should_panic]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert_eq!(result, "Welcome carol!");
        let target = "carol";
        assert!(
            result.contains(target),
            "你的问候中没有包含目标的姓名{},你的问候是`{}`",
            target,
            result,
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn test_internal_adder() {
        assert_eq!(internal_adder(2, 2), 4);
    }


}
