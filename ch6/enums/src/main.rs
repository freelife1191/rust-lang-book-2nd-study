
#[derive(Debug)]
enum IpAddrKind {
    V4, // (u8, u8, u8, u8)  4개의 u8 값을 저장하도록 별도로 설정
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 메서드 본문 정의
        println!("Message: {:?}", self);
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}, loopback: {:?}",home, loopback);

    let m = Message::Write(String::from("hello"));
    // println!("m: {:?}",m)
    m.call();
}
