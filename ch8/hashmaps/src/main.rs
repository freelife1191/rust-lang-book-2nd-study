fn main() {
    // 새로운 해시맵을 생성하여 몇 개의 키와 값을 집어넣기
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 해시맵 내에 저장된 블루 팀의 점수 접근하기
    let team_name = String::from("Blue");
    // get 메서드에 키를 제공하여 해시맵으로부터 값을 얻어올 수 있다
    // get 메서드는 Option<&V>를 반환한다, 해당 키에 대한 값이 없다면 get은 None을 반환
    // copied를 호출하여 Option<&i32>가 아닌 Option<i32>를 얻어온 다음, unwrap_or를 써서 scores가 해당 키에 대한 아이템을 가지고 있지 않을 경우 score에 0을 설정
    // i32처럼 Copy 트레이트를 구현한 타입의 값은 해시맵 안으로 복사됨
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 키와 값이 삽입되는 순간 이들이 해시맵의 소유가 되는 것을 보여주는 예
    // String 처럼 소유권이 있는 값의 경우 값들이 이동되어 해시맵이 그 값의 소유자가 됨
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // insert를 호출하여 field_name과 field_value를 해시맵으로 이동시킨 후에는 더 이상 이 둘을 사용할 수 없다
    map.insert(field_name, field_value);

    // 특정한 키로 저장된 값을 덮어쓰기
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // entry 함수의 반환 값은 열거형 Entry인데, 해당 키가 있는지 혹은 없는지를 나타낸다
    // entry 메서드를 이용하여 어떤 키가 값을 이미 갖고 있지 않을 경우에만 추가하기
    // Entry의 or_insert 메서드는 해당 키가 존재할 경우 Entry 키에 대한 연관된 값을 반환하도록 정의되어 있고,
    // 그렇지 않은 경우 매개변수로 제공된 값을 해당 키에 대한 새 값으로 삽입하고 수정된 Entry에 대한 값을 반환
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 단어와 횟수를 저장하는 해시맵을 사용하여 단어의 등장 횟수 세기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // split_whitespace 메서드는 text의 값을 공백문자로 나눈 서브 슬라이스에 대한 반복자를 반환
    for word in text.split_whitespace() {
        // or_insert 메서드는 실제로는 해당 키에 대한 값의 가변 참조자(&mut V)를 반환
        let count = map.entry(word).or_insert(0);
        // 여기에 값을 할당하기 위해 먼저 애스터리스크(*)를 사용하여 count를 역참조해야 한다
        *count += 1;
    }

    println!("{:?}", map);

}
