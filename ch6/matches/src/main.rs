#[derive(Debug)] // Debug 트레이트를 구현하는 것을 도움
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 열거형의 배리언트에 데이터를 저장하는 것을 도움
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // 열거형과 열거형의 배리언트를 패턴으로 사용하는 match 표현식
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match 표현식은 패턴과 값의 조합을 사용하여 코드를 더 간결하게 만들어 줌
    match x {
        None => None, // Option<T>의 None 값에 대한 패턴 없으면 에러 발생
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let get_coin = value_in_cents(Coin::Penny);
    println!("get_coin: {}", get_coin);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
