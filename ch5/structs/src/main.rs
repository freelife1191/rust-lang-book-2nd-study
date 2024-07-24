fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    println!("email: {}, username: {}, active: {}, sign_in_count: {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let user2 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    println!("email: {}, username: {}, active: {}, sign_in_count: {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count
        ..user1 // 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기
    };
    println!("email: {}, username: {}, active: {}, sign_in_count: {}", user3.email, user3.username, user3.active, user3.sign_in_count);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
