use std::net::IpAddr;

fn main() {
    // panic!("crash and burn");

    // panic!을 일으키는 벡터의 끝을 넘어서는 요소에 대한 접근 시도
    let v = vec![1, 2, 3];
    v[99];

    // expect의 문구에 Err 배리언트가 있으면 안 될 이유를 적어주는 것이 더 좋을 것이다
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}

