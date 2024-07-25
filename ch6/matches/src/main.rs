enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // 열거형과 열거형의 배리언트를 패턴으로 사용하는 match 표현식
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let get_coin = value_in_cents(Coin::Penny);
    println!("get_coin: {}", get_coin)
}
