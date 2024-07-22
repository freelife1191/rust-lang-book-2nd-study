mod function_work;

fn main() {
    // 타입을 명시하지 하지 않으면 에러 발생
    let guess: u32 = "42".parse().expect("Not a number!");

    // 부동소수점 숫자 활용 예제
    let x = 2.0; // f64 1배수의 정밀도인 부동소수점
    let y: f32 = 3.0; // f32 2배수의 정밀도인 부동소수점

    // addition
    let sum = 5 + 10;

    // subtration
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;

    let f: bool = false; // 명시적으로 타입을 지정할 수 있음

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // array
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("The first month is: {}", months[0]);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let index = 10;
    // let element = a[index];
    // println!("The value of first is: {}", first); // 사이즈 초과로 에러 발생


}
