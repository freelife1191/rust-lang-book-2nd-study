#[derive(Debug)] // 외부속성 작성 필요
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    /**
     연관 함수 정의
     정사각형을 만들 때 너비, 높이에 같은 값을 두 번 지정하지 않고 치수 하나를 매개변수로 받아서
     해당 치수로 너비와 높이를 설정하는 연관 함수
     연관 함수 호출: let sq = Rectangle::square(3);
     **/
    fn square(size: u32) -> Self { // Self는 Rectangle
        Self {
            width: size,
            height: size
        }
    }
}
/** 여러 impl 블록을 사용하도록 재작성 **/
impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };
    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    // width 메서드 호출
    if rect1.width() { // 0보다 크면 true 반환
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
    println!("sq width: {}, height: {}", sq.width, sq.height);
}
