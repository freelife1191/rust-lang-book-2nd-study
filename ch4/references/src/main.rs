fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1은 s1의 참조를 의미

    println!("The length of '{}' is {}.", s1, len);

    // 참조 값을 고치려고 하면 오류 발생
    // change(&s1);

    //  가변 참조자 (mutable references)
    let mut s = String::from("hello");
    // 참조 값 변경이 가능
    change(&mut s);
    println!("{}", s);

    // 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있음
    let r1 = &mut s;
    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize { // &String은 String의 참조를 의미
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}