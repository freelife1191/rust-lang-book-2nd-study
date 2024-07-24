/** 구조체를 사용한 예제 프로그램 **/
// 구조체로 리팩터링하여 코드에 더 많은 의미를 담기
// 러스트는 디버깅 정보를 출력하는 기능을 자체적으로 가지고 있음
// 구조체에 해당 기능을 적용하려면 명시적 동의가 필요
#[derive(Debug)] // 외부속성 작성 필요
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    // println!("rect1 is {}", rect1); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    println!("rect1 is {:?}", rect1); // Debug 트레이트를 사용하여 출력하기 더 많은 구조체라면 {:#?} 사용
    // 표현식의 소유권을 가져와서 코드에서 dbg! 매크로를 호출한 파일 및 라인 번호를 결괏값과 함께 출력하고 다시 소유권 반환
    dbg!(&rect1); // dbg!가 rect1의 소유권을 가져가는 것은 원치 않으므로 참조자 사용

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // 튜플 적용 전
    /*
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    */
    // 튜플 적용 후
    /*
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_rect(rect1)
    );
    */
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
/*
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_rect(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/