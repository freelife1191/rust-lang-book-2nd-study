use std::fmt::Display;

/**
두 문자열 슬라이스 중 긴 쪽을 반환하는 longest 함수 (컴파일되지 않음)
반환할 참조자가 x인지, y인지 러스트가 알 수 없기 때문
**/
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
/**
시그니처 내 모든 참조자가 동일한 라이프타임 'a를 가져야 함을 나타낸 longest 함수 정의
**/
/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
/**
longest 함수를 제일 긴 문자열 슬라이스를 반환하는 게 아니라, 항상 첫 번째 매개변수를 반환하도록 변경
**/
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/**
반환할 참조자가 함수 매개변수중 하나를 참조하지 않을 유일한 가능성은 함수 내부에서 만들어진 값의 참조자를 반환하는 경우
값은 함수가 끝나는 시점에 스코프를 벗어나므로 댕글링 참조가 됨
반환 타입에 'a를 지정했지만, 반환 값의 라이프타임이 그 어떤 매개변수와도 관련 없으므로 컴파일할 수 없음
이런 상황을 해결하는 가장 좋은 방법은 참조자 대신 값의 소유권을 갖는 데이터 타입을 반환하여 함수를 호출한 함수 측에서 값을 정리하도록 하는 것
**/
/*
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

/**
ImportantExcerpt 인스턴스는 part 필드가 보관하는 참조자의 라이프타임보다 오래 살 수 없다
**/
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/**
impl 뒤에서 라이프타임 매개변수를 선언하고 타입명 뒤에서 사용하는 과정은 필수적이지만, 첫 번째 생략 규칙으로 인해 self 참조자의 라이프타임을 명시할 필요는 없다
**/
/*
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
*/

/**
두 개의 입력 라이프타임이 있으니, 러스트는 첫 번째 라이프타임 생략 규칙대로 &self, announcement에 각각의 라이프타임을 부여한다
매개변수 중 하나가 &self이니 반환 타입에 &self의 라이프타임을 부여한다
모든 라이프타임이 추론된다
**/
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

/**
제네릭 타입 매개변수, 트레이트 바운드, 라이프타임이 문법이 함수 하나에 전부 들어간 모습
**/
// 라이프타임은 제네릭의 일종이므로, 함수명 뒤의 꺾쇠괄호 안에는 라이프타임 매개변수 'a 선언과 제네릭 타입 매개변수 T가 함께 나열되어 있음
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    // where 구문에 명시한 바와 같이 Display 트레이트를 구현하는 제네릭 타입 T에 해당하는 ann 매개변수를 추가
    // 추가 매개변수는 {}를 사용하여 출력될 것
    ann: T,
) -> &'a str
where
    // Display 트레이트 바운드가 필요
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 라이프타임의 주목적은 댕글링 참조 (dangling reference) 방지
    // 스코프 밖으로 벗어난 값을 참조하는 코드
    /*
    let r;
    {
        let x = 5;
        r = &x;
    }
    // r이 참조하는 값이 사용하려는 시점에 이미 자신의 스코프를 벗어남
    println!("r: {}", r);
    */
    // 데이터의 라이프타임이 참조자의 라이프타임보다 길어서 문제없는 코드
    /*
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    */

    // 두 문자열 슬라이스 중 긴 쪽을 찾기 위해 longest 함수를 호출하는 main 함수
    /*
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    */


    //서로 다른 구체적인 라이프타임을 가진 String 값의 참조자로 longest 함수 호출하기
    /*
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    */

    // string2가 스코프 밖으로 벗어나고 나서 result 사용해 보기
    // 함수 매개변수와 반환 값에 모두 동일한 라이프타임 매개변수 'a를 명시했으므로, 러스트는 문제를 정확히 파악
    // 잠재적으로 유효하지 않은 참조자를 가질 수도 있다고 판단하여 컴파일 실패
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */

    // 참조자를 보유하여 라이프타임 명시가 필요한 구조체
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

}
