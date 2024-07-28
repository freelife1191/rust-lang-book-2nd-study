fn main() {
    /*
    let v = Vec::new();
    let v = vec![1, 2, 3];
    */

    /*
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
    */

    /*
    let v = vec![1, 2, 3, 4, 5];
    // &와 []를 사용하면 인덱스 값에 위치한 요소의 참조자를 얻게 된다
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    // get 함수에 인덱스를 매개변수로 넘기면, match를 통해 처리할 수 있는 Option<&T>를 얻게 된다
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    */

    // let does_not_exist = &v[100];
    // get 함수에 벡터 범위를 벗어난 인덱스가 주어지면 패닉 없이 None이 반환됨
    // let does_not_exist = v.get(100);

    // 벡터는 모든 요소가 서로 붙어서 메모리에 저장됨
    // 새로운 요소를 벡터 끝에 추가할 경우, 현재 벡터 메모리 위치에 새로운 요소를 추가할 공간이 없다면, 다른 넉넉한 곳에 메모리를 새로 할당하고 기존 요소를 새로 할당한 공간에 복사
    // 기존 요소의 참조자는 해제된 메모리를 가리키게 되기 때문에, 이러한 상황을 대여 규칙으로 막아둔 것
    /*
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // ----- immutable borrow later used here
    println!("The first element is: {first}");
    */

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // 가변 참조자가 가리키는 값을 수정하려면, += 연산자를 쓰기 전에 * 역참조 연산자로 i의 값을 얻어야 한다
        *i += 50;
        println!("{i}");
    }

    // 열거형을 정의하여 백터 내에 다른 타입의 데이터를 담을 수 있도록 한다
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
