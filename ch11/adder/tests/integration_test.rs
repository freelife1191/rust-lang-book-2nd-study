use adder;

mod common;

// adder 크레이트 내 함수를 테스트하는 통합 테스트
/**
tests/integration_test.rs 내 코드는 #[cfg(test)]가 필요 없다
카고는 tests 디렉터리를 특별 취급하여, 디렉터리 내 파일을 cargo test 시에만 컴파일한다
**/
#[test]
fn it_adds_two() {
    // `mod common;` 를 선언하고 나면 `common::setup()` 함수를 호출
    common::setup();
    assert_eq!(4, adder::add_two(2));
}