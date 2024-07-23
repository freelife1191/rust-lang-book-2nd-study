fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1은 s1의 참조를 의미

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // &String은 String의 참조를 의미
    s.len()
}
