fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number { // bool을 기대하지만 정수가 와서 에러 발생
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    // let number = if condition { 5 } else { "six" }; // 타입이 맞지 않아 에러 발생
    println!("The value of number is: {}", number);

    loop {
        println!("again!");
        break; // 무한 루프를 방지하기 위해 break를 사용
    }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() { // 배열의 요소를 순회하며 출력
        println!("the value is: {}", element);
    }


    for number in (1..4).rev() { // 1부터 3까지 역순으로 출력
        println!("{}!", number);
    }
}
