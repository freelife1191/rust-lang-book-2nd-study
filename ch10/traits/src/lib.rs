use std::fmt::Display;

// summarize 메서드가 제공하는 동작으로 구성된 Summary 트레이트
/*
pub trait Summary {
    fn summarize(&self) -> String;
}
*/
/*
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
*/
// summarize 메서드의 기본 구현 내에서 summarize_author 메서드를 호출
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// NewsArticle과 Tweet 타입에 Summary 트레이트 구현하기
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


/**
NewsArticle, Tweet 타입에 구현한 Summary 트레이트를 사용하여
Summary 트레이트를 구현하는 어떤 타입의 item 매개변수에서 summarize 메서드를 호출하는 notify 함수를 정의
**/
// 트레이트 바운드 문법
// pub fn notify(item1: &impl Summary, item2: &impl Summary) { // Summary를 구현하는 두 매개변수를 전달받는 함수를 구현
// pub fn notify<T: Summary>(item1: &T, item2: &T) { // 두 매개변수가 같은 타입으로 강제
// + 구문으로 트레이트 바운드를 여럿 지정하기
// pub fn notify(item: &(impl Summary + Display)) { // notify의 정의를 할때 item이 Display, Summary를 모두 구현해야 하도록 지정
// pub fn notify<T: Summary + Display>(item: &T) { // 제네릭 타입의 트레이트 바운드에도 사용
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// where 조항으로 트레이트 바운드 정리하기
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
/*
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
*/
/**
트레이트를 구현하는 타입을 반환하기
**/
/*
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
*/
/**
반환형을 impl Summary로 지정하고 NewsArticle, Tweet 중 하나를 반환하는 코드 예시
컴파일 X
NewsArticle, Tweet 중 하나를 반환하는 행위는 impl Trait 문법이 컴파일러 내에 구현된 방식으로 인한 제약 때문에 허용되지 않음
**/
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
*/

// 트레이트 바운드를 이용해 제네릭 타입에 조건부로 메서드 구현하기
// 트레이트 바운드를 만족하는 모든 타입에 대해 트레이트를 구현하는 것을 포괄 구현(blanket implementations) 이라 하며
// 이는 러스트 표준 라이브러리 내에서 광범위하게 사용됨
/*
Display 트레이트가 구현된 모든 타입에서 (ToString 트레이트에 정의된) to_string() 메서드를 호출할 수 있는 건 표준 라이브러리의 이 포괄 구현 덕분
impl<T: Display> ToString for T {
    // --생략--
}
let s = 3.to_string();
*/
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}