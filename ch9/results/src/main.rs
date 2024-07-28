use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, self, Read};

/*
fn main() {
    let greeting_file_result = File::open("hello.txt");
    // match 표현식을 사용하여 반환 가능한 Result 배리언트들을 처리하기
    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    */
    // 다른 종류의 에러를 다른 방식으로 처리하기
    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // io::ErrorKind는 io 연산으로부터 발생할 수 있는 다양한 종류의 에러를 나타내는 배리언트가 있는 열거형
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    */

    // unwrap_or_else 메서드와 클로저를 사용
    // 에러를 다룰 때 이런 메서드를 사용하면 거대하게 중첩된 match 표현식 덩어리를 제거할 수 있다
    /*
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */

    /*
    // unwrap 메서드에 의해 호출된 panic!으로부터의 에러 메시지를 보게 됨
    let greeting_file = File::open("hello.txt").unwrap();
    // unwrap은 panic!의 기본 메시지가 출력되지만, expect는 매개변수로 전달한 에러 메시지를 출력
    // 프로덕션급 품질의 코드에서 대부분의 러스타시안은 unwrap보다 expect를 선택하여 해당 연산이 항시 성공한다고 기대하는 이유에 대한 더 많은 맥락을 제공
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
    */

    // ()를 반환하는 main에서의 ? 사용 시도는 컴파일되지 않습니다
    // ? 연산자는 Result, Option 혹은 FromResidual을 구현한 타입을 반환하는 함수에서만 사용될 수 있음
    // let greeting_file = File::open("hello.txt")?;
}
*/

// match를 이용하여 호출 코드 쪽으로 에러를 반환하는 함수
/*
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
*/

// main이 Result<(), E>를 반환하도록 하여 Result 값에 대한 ? 사용 가능하게 하기
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

// ? 연산자를 이용하여 에러를 호출 코드 쪽으로 반환하는 함수
// ? 연산자를 사용할 때의 에러 값들은 from 함수를 거친다
// from 함수는 표준 라이브러리 내의 From 트레이트에 정의되어 있으며 어떤 값의 타입을 다른 타입으로 변환하는 데에 사용된다
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;

    /*
    let mut username = String::new();
    // ? 연산자 뒤에 메서드 호출을 연결하기
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
    */
    // 파일을 열고, 읽는 대신 fs::read_to_string을 사용하기
    fs::read_to_string("hello.txt")
}

// 주어진 텍스트에서 첫 번째 줄의 마지막 문자를 찾는 함수의 예제
// Result에 ?를 사용할 때와 마찬가지로, 함수가 Option를 반환하는 경우에는 Option에서만 ?를 사용할 수 있다
// Option<T> 값에 대한 ? 연산자의 사용
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}