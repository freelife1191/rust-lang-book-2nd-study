# The Rust Programming Language Second Edition

## Rust 문서

- 러스트 북 커뮤니티 번역본: https://doc.rust-kr.org/
- 러스트 북 원본 영문: https://doc.rust-lang.org/stable/book/
- 러스트 북 Summary: https://codeahoy.com/learn/tutorials/rust-book-summary/
- 러스트 북 Example Solutions
  - https://github.com/kmoschcau/rust-book-exercises
  - https://github.com/olehmisar/The-Rust-Programming-Language-Book-Solutions
  - https://github.com/laercioxlaercio/rust
  - https://github.com/kevinalh/rust-book
  - https://github.com/klemola/rust-book
  - https://github.com/jasonkuhrt-archive/rust-book-exercises
  - https://github.com/Lukman-01/rust-learn-by-practice
  - https://github.com/rust-unofficial/rust-practise-questions/tree/master/src
- 러스트 북 Quiz 버전: https://rust-book.cs.brown.edu/
- 표준 라이브러리 문서: https://www.rust-lang.org/learn
- Rust API Document: https://doc.rust-lang.org/std/
- Rust Project: https://www.rust-lang.org/
- Rust Document: https://doc.rust-lang.org/beta/
  - Rust Reference: https://doc.rust-lang.org/beta/reference/index.html
  - The Edition Guide: https://doc.rust-lang.org/beta/edition-guide/editions/index.html
  - The Release Notes: https://doc.rust-lang.org/beta/releases.html
  - The rustc Book: https://doc.rust-lang.org/rustc/
  - The Cargo Book: https://doc.rust-lang.org/beta/cargo/index.html
  - The Rustdoc Book: https://doc.rust-lang.org/beta/rustdoc/index.html
  - The Clippy Book: https://doc.rust-lang.org/beta/clippy/index.html
  - rustc error codes: https://doc.rust-lang.org/beta/error_codes/index.html
  - The Style Guide: https://doc.rust-lang.org/beta/style-guide/index.html
  - The Rustonomicon: https://doc.rust-lang.org/beta/nomicon/index.html
  - The Unstable Book: https://doc.rust-lang.org/beta/unstable-book/index.html
  - Rust Compiler Developer Guide: https://rustc-dev-guide.rust-lang.org/
  - Rust Embedded: https://github.com/rust-embedded
  - The Embedded Rust Book: https://doc.rust-lang.org/beta/embedded-book/index.html
- Rust Docs 검색: https://docs.rs/
- Rust Playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
- Rust 예제: https://doc.rust-lang.org/rust-by-example/
- Rust 예제 & 해답: https://github.com/rust-lang/rustlings
  - https://github.imc.re/topics/rustlings-solution?o=asc&s=stars

## Ownership
러스트의 가장 유니크한 특성, 러스트가 가비지 콜렉터 없이 메모리 안정성 보장을 하게 해준다

러스트는 제3의 접근법을 이용한다
메모리는 컴파일 타임에 컴파일러가 체크할 규칙들로 구성된 소유권 시스템을 통해 관리된다
소유권 기능들의 어떤 것도 런타임 비용이 발생하지 않는다

#### 스택과 힙

- Stack: LIFO(Last In First Out) 구조, 빠르지만 크기가 컴파일 타임에 결정된다
  - 데이터를 추가하는 것 스택에 푸시하기(pushing on to the stack)
  - 데이터를 제거하는 것 스택에서 팝하기(popping off the stack)
- Heap: 느리지만 크기가 런타임에 결정된다

스택은 데이터에 접근하는 방식 덕택에 빠르다
새로운 데이터를 넣어두기 위한 공간 혹은 데이터를 가져올 공간을 검색할 필요가 전혀 없는데 바로 그 공간이 항상 스택의 꼭대기이기 때문이다
스택에 담긴 모든 데이터가 결정되어 있는 고정된 크기를 갖고 있어야 한다

컴파일 타임에 크기가 결정되어 있지 않거나 크기가 변경될 수 있는 데이터를 위해서는, 힙에 데이터를 저장할 수 있다
데이터를 힙에 넣을때, 먼저 저장할 공간이 있는지 묻는다
운영체제가 충분히 커다란 힙 안의 빈 어떤 지점을 찾아서 이 곳을 사용중이라고 표시하고, 해당 지점의 포인터를 우리에게 돌려준다
이 절차를 힙 공간 할당하기(allocating on the heap)라고 부르고, 종종 그냥 할당(allocating)으로 줄여 부른다
포인터는 결정되어 있는 고정된 크기의 값이므로, 스택에 포인터를 저장할 수 있지만, 실제 데이터를 사용하고자 할 때는 포인터를 따라가야 한다

힙에 저장된 데이터에 접근하는 것은 스택에 저장된 데이터에 접근하는 것보다 느린데, 그이유는 포인터가 가리킨 곳을 따라가야 하기 때문이다
프로세서는 (힙에 있는 데이터와 같이) 멀리 떨어져 있는 데이터들 보다는 (스택에 있는 것과 같이) 붙어있는 데이터들에 대한 작업을 하면 더 빨라진다
힙으로부터 큰 공간을 할당받는것 또한 시간이 걸릴 수 있다

소유권과 관계된 문제들
- 코드의 어느 부분이 힙의 어떤 데이터를 사용하는지 추적하는 것
- 힙의 중복된 데이터의 양을 최소화하는 것
- 힙 내에 사용하지 않는 데이터를 제거하여 공간이 모자라지 않게 하는 것

힙 데이터를 관리하는 것이 곧 소유권의 존재 이유이다

#### 소유권 규칙
- 러스트의 각각의 값은 해당값의 소유자(owner)라고 불리우는 변수를 갖고 있다
- 한번에 딱 하나의 오너만 존재할 수 있다
- 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped)

#### String 타입

String Literal로 부터 `from` 이라는 함수를 이용해서 `String`을 만들 수 있다

```rust
let s = String::from("hello");
```

더블 콜론(`::`)은 `String` 타입 아래의 `from` 함수를 특정지을 수 있도록 해주는 네임스페이스 연산자이다

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str()은 해당 스트링 리터럴을 스트링에 붙여준다
println!("{}", s); // 이 부분이 `hello, world!`를 출력한다
```

#### 메모리와 할당

러스트는 메모리는 변수가 소속되어 있는 스코프 밖으로 벗어나는 순간 자동으로 반납된다

```rust
{
    let s = String::from("hello"); // s는 이 지점부터 유효하다
    // s를 사용한다
} // 이 스코프는 끝났고, s는 더이상 유효하지 않다
```

#### 가변 참조자(Mutable References)

- 가변 참조자의 딱 한가지 큰 제한
  - 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

#### 데이터 레이스(data race)

1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다
2. 그 중 적어도 하나의 포인터가 데이터를 쓴다
3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다

러스트는 이러한 데이터 레이스를 방지하기 위해 컴파일 타임에 가변 참조자를 사용하는 규칙을 강제한다  

불변 참조자를 가지고 있을 동안에도 가변 참조자를 만들 수 없음

#### 댕글링 포인터(dangling pointer)

댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안,  
그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터 

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

해결법 `String`을 직접 반환

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

### 슬라이스

#### 스트링 슬라이스

`String`의 참조자 대신 스트링 슬라이스를 갖도록 정의하는 것은 API를 어떠한 기능적인 손실 없이도 더 일반적이고 유용하게 해준다

소유권, 빌림, 그리고 슬라이스의 개념은 러스트 프로그램의 메모리 안정성을 컴파일 타임에 보장하는 것이다
러스트 언어는 다른 시스템 프로그래밍 언어와 같이 메모리 사용에 대한 제어권을 주지만, 데이터의 소유자가 스코프 밖으로 벗어났을 때  
소유자가 자동적으로 데이터를 버리도록 하는 것은 이러한 제어를 위해 추가적인 코드 작성이나 디버깅을 하지 않아도 된다는 뜻이다


## 7.1 패키지와 크레이트

러스트에는 코드 조직화에 필요한 기능이 여럿 있다
어떤 세부 정보를 외부에 노출할지, 비공개로 둘지, 프로그램의 스코프 내 어떤 이름이 있는지 등 다양하다
이를 통틀어 모듈 시스템이라 하며, 다음 기능들이 포함된다

- 패키지: 크레이트를 빌드하고, 테스트하고, 공유하는 데 사용하는 카고 기능
  - 일련의 기능을 제공하는 하나 이상의 크레이트로 구성된 번들
  - 크레이트들을 빌드하는 법이 설명된 `Cargo.toml` 파일이 포함되어 있다
  - `src/main.rs` 는 크레이트 루트라는 관례를 준수
  - `src/lib.rs` 는 파일이 존재할 경우, 카고는 해당 패키지가 패키지명과 같은 이름의 라이브러리 크레이트를 포함하고 있다고 판단
  - `src/bin` 디렉터리 내에 파일을 배치하면 각각의 파일이 바이너리 크레이트가 되어, 여러 바이너리 크레이트를 패키지에 포함할 수 있다
- 크레이트: 라이브러리나 실행 가능한 모듈로 구성된 트리 구조
  - 러스트가 컴파일 한 차례에 고려하는 가장 작은 코드 단위
  - 바이너리일 수도 있고, 라이브러리일 수도 있다
  - 라이브러리 크레이트는 `main` 함수를 가지고 있지 않고 실행파일 형태로 컴파일되지 않는다
- 모듈과 `use`: 구조, 스코프를 제어하고, 조직 세부 경로를 감추는 데 사용함
  - 스코프에 경로를 가져오는 `use` 키워드
  - `mod` 키워드로 모듈을 정의하고, `mod.rs` 파일을 사용하여 모듈을 관리함
  - `crate` 키워드로 크레이트 루트를 가리킴
  - `super` 키워드로 부모 모듈을 가리킴
  - `self` 키워드로 현재 모듈을 가리킴
  - `path` 키워드로 경로를 정의함
  - `as` 키워드로 이름을 다시 정의함
  - `pub use` 키워드로 다른 모듈의 아이템을 공개함
  - `pub` 키워드로 아이템을 공개함
- 경로: 구조체, 함수 모듈 등의 이름을 지정함
    - 절대 경로: 크레이트 루트부터 시작하는 경로
    - 상대 경로: 현재 모듈부터 시작하는 경로

#### 모듈 치트 시트

- 크레이트 루트부터 시작: 크레이트를 컴파일할 때 컴파일러는 먼저 크레이트 루트 파일을 본다
  - 라이브러리 크레이트: `src/lib.rs`
  - 바이너리 크레이트: `src/main.rs`
- 모듈 선언: 크레이트 루트 파일에 `mod` 선언을 추가하여 모듈을 선언한다
- 서브모듈 선언: 크레이트 루트가 아닌 다른 파일에서는 서브모듈을 선언할 수 있다
- 모듈 내 코드로의 경로: 일단 모듈이 크레이트의 일부로서 구성되면, 공개 규칙이 허용하는 한도 내에서라면 해당 코드의 경로를 사용하여 동일한 크레이트의 어디에서든 이 모듈의 코드를 참조할 수 있게 된다
  - 예를 들면, garden vegetables 모듈 안에 있는 Asparagus 타입은 `crate::garden::vegetables::Asparagus`로 찾아 쓸 수 있다
- 비공개 VS 공개: 모듈 내의 코드는 기본적으로 부모 모듈에게 비공개(private) 이다
  - 모듈을 공개 (public)로 만들려면, `mod` 대신 `pub mod`를 사용한다
  - 공개 모듈의 아이템들을 공개하려면 `pub` 키워드를 사용한다
- `use` 키워드: 어떤 스코프 내에서 `use` 키워드는 긴 경로의 반복을 줄이기 위한 어떤 아이템으로의 단축경로를 만들어 준다
  - `crate::garden::vegetables::Asparagus`를 참조할 수 있는 모든 스코프에서 `use crate::garden::vegetables::Asparagus;`로 단축경로를 만들 수 있다
  - 그 이후부터는 스코프에서 이 타입을 사용하려면 Asparagus 만 작성해주면 된다

## 7.3 경로를 사용하여 모듈 트리의 아이템 참조하기

---

- 절대 경로(absolute path): 크레이트 루트부터 시작하는 경로
  - `crate` - 크레이트 루트
- 상대 경로(relative path): 현재 모듈부터 시작하는 경로
  - `self` - 현재 모듈
  - `super` - 부모 모듈


## 에러 처리

---

대부분의 언어는 예외 처리 (exception) 와 같은 메커니즘을 이용하여 이 두 종류의 에러를 구분하지 않고 같은 방식으로 처리한다  
러스트에는 예외 처리 기능이 없다  
대신, 복구 가능한 에러를 위한 `Result<T, E>` 타입과 복구 불가능한 에러가 발생했을 때 프로그램을 종료하는 `panic!` 매크로가 있다

기본적으로 이러한 패닉은 실패 메시지를 출력하고, 되감고(unwind), 스택을 청소하고, 종료한다  
패닉이 발생했을 때 그 패닉의 근원을 쉽게 추적하기 위해 환경 변수를 통하여 러스트가 호출 스택을 보여주도록 할 수 있다
러스트에서는 프로그램이 데이터 정리 작업 없이 즉각 종료되는 대안인 **그만두기(aborting)** 를 선택할 수도 있다

`Cargo.toml` 내에서 적합한 `[profile]` 섹션에 `panic = 'abort'`를 추가하여 되감기를 그만두기로 바꿀 수 있다

```properties
[profile.release]
panic = 'abort'
```

패닉이 발생했을 때 호출 스택을 출력하려면 `RUST_BACKTRACE=1` 환경 변수를 설정한다
```shell
$ RUST_BACKTRACE=1 cargo run
```


실패할지도 모르는 함수를 정의할 때는 기본적으로 Result를 반환하는 것이 좋은 선택  
예제, 프로토타입, 테스트 같은 상황에서는 Result를 반환하는 대신 패닉을 일으키는 코드가 더 적절함

#### 에러 처리를 위한 가이드라인

- 이 나쁜 상태란 것은 예기치 못한 무언가이며, 이는 사용자가 입력한 데이터가 잘못된 형식이라던가 하는 흔히 발생할 수 있는 것과는 반대되는 것이다
- 그 시점 이후의 코드는 매번 해당 문제에 대한 검사를 하는 것이 아니라, 이 나쁜 상태에 있지 않아야만 할 필요가 있다
- 사용하고 있는 타입 내에 이 정보를 집어넣을만한 뾰족한 수가 없다

#### 유효성을 위한 커스텀 타입 생성하기

```rust
pub struct Guess {
  value: i32,
}

// 1과 100 사이의 값일 때만 실행을 계속하는 Guess 타입
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }

    Guess { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}
```

## 제네릭 타입, 트레이트, 라이프타임

---

- 중복된 코드를 식별
- 중복된 코드를 함수의 본문으로 분리하고, 함수의 시그니처 내에 해당 코드의 입력값 및 반환 값을 명시
- 중복됐었던 두 지점의 코드를 함수 호출로 변경

#### 제네릭 코드의 성능

러스트는 컴파일 타임에 제네릭을 사용하는 코드를 단형성화(monomorphization) 한다  
단형성화란 제네릭 코드를 실제 구체 타입으로 채워진 특정한 코드로 바꾸는 과정을 말한다

단형성화 과정은 러스트 제네릭을 런타임에 극도로 효율적으로 만들어준다

#### 트레이트로 공통된 동작을 정의하기

- 트레이트(trait) 는 특정한 타입이 가지고 있으면서 다른 타입과 공유할 수 있는 기능을 정의한다
- 트레이트를 사용하면 공통된 기능을 추상적으로 정의할 수 있다
- 트레이트 바운드(trait bound) 를 이용하면 어떤 제네릭 타입 자리에 특정한 동작을 갖춘 타입이 올 수 있음을 명시할 수 있다

약간의 차이는 있으나, 트레이트는 다른 언어에서 흔히 인터페이스 (interface) 라고 부르는 기능과 유사하다

#### 라이프타임 명시 문법

`&i32`        // 참조자
`&'a i32`     // 명시적인 라이프타임이 있는 참조자
`&'a mut i32` // 명시적인 라이프타임이 있는 가변 참조자

#### 정적 라이프 타임

정적 라이프타임 (static lifetime), `static`이라는 특별한 라이프타임  
`static` 라이프타임은 해당 참조자가 프로그램의 전체 생애주기 동안 살아있음을 의미  
모든 문자열 리터럴은 `static` 라이프타임을 가지며, 다음과 같이 명시할 수 있다

```rust
let s: &'static str = "I have a static lifetime.";
```

## 11. 자동화 테스트 작성하기

---

### 테스트 작성 방법

#### assert! 매크로 결과 검사하기
어떤 조건이 true임을 보장하는 테스트를 작성할 땐 표준 라이브러리가 제공하는 assert! 매크로가 유용  
`assert_eq!`, `assert_ne!` 매크로는 각각 두 인수를 비교하고 동등한지 (equality) 그렇지 않은지 (inequality) 판단  

내부적으로 `assert_eq!`, `assert_ne!` 매크로는 각각 ==, != 연산자를 사용  
매크로는 인수를 디버그 형식으로 출력하는데, 즉 `assert_eq!`, `assert_ne!` 매크로로 비교할 값은 `PartialEq`, `Debug` 트레이트를 구현해야 한다  
`#[derive(PartialEq, Debug)]`를 어노테이션하는 것이 일반적


### 테스트 실행 방법 제어하기

#### 테스트를 병렬 혹은 순차적으로 실행하기

테스트를 병렬로 실행하고 싶지 않거나, 사용할 스레드의 개수에 대한 미세 조정이 필요한 경우에는 `--test-threads` 플래그와 함께 테스트 바이너리에서 사용할 스레드 개수를 지정할 수 있다

스레드 개수를 1로 설정하여 프로그램이 어떠한 병렬 처리도 사용하지 않도록 하였다
```shell
$ cargo test -- --test-threads=1
```

#### 함수 출력 표시하기

기본적으로, 러스트 테스트 라이브러리는 성공한 테스트의 모든 표준 출력(standard output) 을 캡처한다  
테스트에서 `println!` 매크로를 호출해도, 해당 테스트가 성공하면 터미널에서 `println!`의 출력을 찾아볼 수 없다

`println!`을 호출하는 함수 테스트
```rust
fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

#[cfg(test)]
mod tests {
  use super::*;

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
}
```

성공한 테스트에서 출력한 내용도 보고 싶다면, 러스트에게 `--show-output` 옵션을 전달하여 성공한 테스트의 출력도 표시하도록 할 수 있다

```shell
$ cargo test -- --show-output
```

테스트를 아무 인수도 없이 실행하면 모든 테스트가 병렬로 실행된다

#### 테스트 하나만 실행하기

```shell
$ cargo test one_hundred
```

#### 테스트를 필터링하여 여러 테스트 실행하기

```shell
$ cargo test add
```

#### 특별 요청이 없다면 일부 테스트 무시하기

`cargo test -- --ignored` 명령어를 사용하면 무시된 테스트만 실행할 수 있다  
무시되었건 말건 간에 모든 테스트를 실행하고 싶다면 `cargo test -- --include-ignored`를 실행

```rust
#[test]
fn it_works() {
  assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

### 테스트 조직화

#### 유닛 테스트

`#[cfg(test)]`은 이 코드가 cargo build 명령어가 아닌 cargo test 명령어 실행 시에만 컴파일 및 실행될 것을 러스트에게 전달한다  
라이브러리 빌드 시 테스트 코드는 제외되므로, 컴파일 소요 시간이 짧아지고, 컴파일 결과물 크기도 줄어든다

비공개 함수 테스트하기
```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

#### 통합 테스트

프로젝트 디렉터리 최상위, 다시 말해 src 옆에 tests 디렉터리를 생성한다  
카고는 디렉터리 내 통합 테스트 파일을 자동으로 인식한다