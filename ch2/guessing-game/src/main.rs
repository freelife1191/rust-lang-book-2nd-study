extern crate rand; // 외부에 의존하는 크레이트가 있음

use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // 함ㅜ는 OS가 시드(seed)르ㄹ 정하고 현재 스레드에서만 사용되는 특별한 정수생성기를 돌려 준다
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // mut 는 mutable
        let mut guess = String::new();
        // 사용자 입력을 받고 결과값을 표시
        // read_line은 사용자가 입력할 때 마다 입력된 문자들을 하나의 문자열에 저장, 가변이어야 됨
        // & 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법 제공 참조자
        io::stdin().read_line(&mut guess) // 참조자를 가변으로 바꿈
            // io::Result 인스턴스가 Err일 경우 작동을 멈추고 메세지 출력
            .expect("Failed to read line");

        // match 에러 발생 시 종료에서 처리로 변경
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            // .expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}