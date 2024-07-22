fn main() {
    let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;

    const MAX_POINTS: u32 = 100_000;
    println!("The value of const is: {}", MAX_POINTS);

    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", x)
    // spaces = spaces.len(); // mismatched types
}