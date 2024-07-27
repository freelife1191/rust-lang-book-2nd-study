use crate::Coin::Dime;

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
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    count
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let value = value_in_cents(Coin::Penny);
    println!("value: {}", value);
}
