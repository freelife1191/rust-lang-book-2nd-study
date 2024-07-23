fn main() {
    // let mut s = String::from("hello world");
    // let first_word = first_word(&s);
    // let second_word = second_word(&s);
    // println!("{} {}", first_word, second_word);
    // s.clear();
    let my_string = String::from("hello world");
    // first_word가 String의 슬라이스로 동작한다
    let word = first_word(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작한다
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 떄문에,
    // 아래 코드도 슬라이스 문법 없이 동작한다
    let word = first_word(my_string_literal);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
/*
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start != 0 {
                end = i;
                break;
            } else {
                start = i + 1;
            }
        }
    }

    if end == 0 {
        end = s.len();
    }

    &s[start..end]
}
*/