fn five() -> i32 {
    5 // 반환값이 있는 함수
}

fn main() {
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1 // 마지막 표현식이 반환값이 됨
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    // x + 1; // 세미콜론을 붙이지 않아 에러 발생
    x + 1
}