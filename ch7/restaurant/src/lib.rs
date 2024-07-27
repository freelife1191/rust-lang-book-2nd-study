mod front_of_house;

fn deliver_order() {}
mod back_of_house {
    // pub를 쓰면 구조체는 공개되지만, 구조체의 필드는 비공개로 유지된다
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    /**
    열거형의 베리언트는 기본적으로 공개
    **/
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // super 는 back_of_house 의 부모 모듈 즉 루트를 의미함
        super::deliver_order();
    }

    fn cook_order() {}
}

mod customer {
    // 단축경로를 만드는 use 키워드 파일 시스템에서 심볼릭 링크를 생성하는 것과 유사
    // use crate::front_of_house::hosting;
    // 다시 내보내기(re-exporting)
    pub use crate::front_of_house::hosting;
    use std::collections::HashMap;

    pub fn eat_at_restaurant() {
        // 절대 경로
        // crate::front_of_house::hosting::add_to_waitlist();
        hosting::add_to_waitlist();

        // 상대 경로
        super::front_of_house::hosting::add_to_waitlist();

        // 호밀 (Rye) 토스트를 곁들인 여름철 조식 주문하기
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // 먹고 싶은 빵 바꾸기
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // 다음 라인의 주석을 해제하면 컴파일되지 않습니다; 식사와 함께
        // 제공되는 계절 과일은 조회나 수정이 허용되지 않습니다
        // meal.seasonal_fruit = String::from("blueberries");
        // 열거형과 열거형의 모든 배리언트를 공개로 지정하기
        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
        println!("{:?}, {:?}", order1, order2);

        let mut map = HashMap::new();
        map.insert(1, 2);
        println!("{:?}", map);
    }
}

use std::fmt::Result;
// 상단에 같은 Result 와 충돌 방지
use std::io::Result as IoResult;
// std 표준 라이브러리
// use std::cmp::Ordering;
// use std::io;
// 중첩 경로 사용
// use std::{cmp::Ordering, io};
// 한줄로 두개의 스코프 가져오기
use std::io::{self, Write};
// glob 연산자
use std::collections::*;
fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}