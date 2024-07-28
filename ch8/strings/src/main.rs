fn main() {
    let mut s = String::new();

    // to_string 메서드를 사용하여 문자열 리터럴로부터 String 생성하기
    let data = "initial contents";
    let s = data.to_string();
    // 이 메서드는 리터럴에서도 바로 작동합니다:
    let s = "initial contents".to_string();

    // String::from 함수를 사용하여 문자열 리터럴로부터 String 생성하기
    let s = String::from("initial contents");

    // 문자열에 다양한 언어로 인사말 저장하기
    // 문자열이 UTF-8으로 인코딩되었음
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // push_str 메서드를 사용하여 String에 문자열 슬라이스 추가하기
    let mut s = String::from("foo");
    s.push_str("bar");

    // 문자열 슬라이스를 String에 붙인 이후에 문자열 슬라이스를 사용하기
    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str 메서드는 문자열 슬라이스를 매개변수로 갖는데 이는 매개변수의 소유권을 가져올 필요가 없기 때문
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push를 사용하여 String 값에 한 글자 추가하기
    let mut s = String::from("lo");
    s.push('l');

    // + 연산자를 사용하여 두 String 값을 하나의 새로운 String 값으로 조합하기
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1이 더하기 연산 이후에 더 이상 유효하지 않은 이유와 s2의 참조자가 사용되는 이유는 + 연산자를 사용했을 때 호출되는 함수의 시그니처와 맞춰야 하기 때문
    // fn add(self, s: &str) -> String {
    // 이러한 구현은 복사보다 더 효율적
    let s3 = s1 + &s2; // s1은 여기로 이동되어 더 이상 사용할 수 없음을 주의하세요

    // 여러 문자열을 접하고자 한다면, +의 동작은 다루기 불편해 짐
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // 더 복잡한 문자열 조합에는 대신 format! 매크로를 사용
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! 매크로로 만들어진 코드는 참조자를 이용하므로 이 호출은 아무 매개변수의 소유권도 가져가지 않음
    let s = format!("{s1}-{s2}-{s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
