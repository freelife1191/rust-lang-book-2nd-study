/**
4장에서 정의했던, 매개변수, 반환 타입이 참조자인데도 라이프타임 명시 없이 컴파일 가능한 함수
러스트 팀은 컴파일러 내에 이 패턴들을 프로그래밍하여, 이러한 상황들에서는 라이프타임을 명시하지 않아도 대여 검사기가 추론할 수 있도록 하였다
**/
// fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
