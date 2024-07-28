/*
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// 제네릭 타입 매개변수를 이용한 largest 함수; 아직 컴파일되지는 않습니다
/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// T 타입의 값 x, y를 갖는 Point<T> 구조체
/*
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
*/
/*
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
*/

/*
struct Point<T, U> {
    x: T,
    y: U,
}
*/
/*
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

// 이름과 타입 시그니처만 다른 두 함수
fn main() {
    /*
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    */

    // 컴파일 안됨
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // x와 y 필드는 둘 다 동일한 제네릭 데이터 타입 T이므로 같은 타입이어야 함
    // let wont_work = Point { x: 5, y: 4.0 };

    // 두 타입의 제네릭을 사용하여, x와 y가 서로 다른 타입의 값이 될 수 있는 Point<T, U>
    // let both_integer = Point { x: 5, y: 10 };
    // let both_float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    // T 타입의 x 필드에 대한 참조자를 반환하는 x 메서드를 Point<T>에 정의
    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    // 구조체 정의와 다른 제네릭 타입을 사용하는 메서드
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // 러스트 컴파일러가 제네릭 코드를 각 인스턴스의 명시적인 타입으로 변경해 주는 덕분에
    // 굳이 런타임 비용을 줄이기 위해 수동으로 직접 각 타입마다 중복된 코드를 작성할 필요가 없다
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
