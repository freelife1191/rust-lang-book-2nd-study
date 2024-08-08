/**
Rectangle 구조체와 can_hold 메서드
**/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
    // a + 3
}

pub fn greeting(name: &str) -> String {
    // String::from("Hello!") // 의도적 버그 발생
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        // if value < 1 { // 의도적 버그 발생
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                // Guess value must be greater than or equal to 1, got 200. 메세지를 단서로 버그 찾기
                // "Guess value must be less than or equal to 100, got {}.", // 의도적 버그 발생
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                // "Guess value must be greater than or equal to 1, got {}.", // 의도적 버그 발생
                value
            );
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}


#[cfg(test)]
mod tests {
    use super::*;

    // 큰 사각형이 작은 사각형을 정말로 포함할 수 있는지 검사하는 can_hold 메서드 테스트
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));
        // 실제 테스트 결괏값을 볼 수 있으니 의도했던 것과 무엇이 다른지 알 수 있어, 디버깅하는 데 도움이 된다
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // 패닉 검사 테스트 함수에는 should_panic 속성을 추가한다
    // 이 테스트는 내부에서 패닉이 발생해야 통과되고, 패닉이 발생하지 않으면 실패한다
    // #[should_panic]
    // 특정한 부분 문자열을 포함하는 패닉 메시지를 사용한 panic!에 대한 테스트
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Result<T, E>를 이용한 테스트
    // Result<T, E> 테스트에서는 #[should_panic] 어노테이션을 사용할 수 없다
    // 연산이 Err 배리언트를 반환하는 것을 단언하기 위해서는 Result<T, E> 값에 물음표 연산자를 사용하지 말고 assert!(value.is_err())를 사용한다
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // println!을 호출하는 함수 테스트
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // 세 가지 서로 다른 이름의 테스트
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
