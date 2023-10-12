#![deny(clippy::all)]

fn main() {
    // Expressions in Rust are almost the same as they are in C#.
    // Arithmetic expressions
    let addition: i32 = 5 + 3;
    let subtraction: i32 = 10 - 4;
    let multiplication: i32 = 6 * 2;
    let division: i32 = 8 / 2;
    let remainder: i32 = 15 % 4;

    println!("Addition: {}", addition);
    println!("Subtraction: {}", subtraction);
    println!("Multiplication: {}", multiplication);
    println!("Division: {}", division);
    println!("Remainder: {}", remainder);

    // Boolean expressions
    let is_true: bool = true;
    let is_false: bool = false;

    // Comparison expressions
    let equal: bool = 5 == 5;
    let not_equal: bool = 10 != 5;
    let greater_than: bool = 8 > 5;
    let less_than: bool = 8 < 5;
    let less_than_or_equal: bool = 3 <= 3;

    println!("Is true: {}", is_true);
    println!("Is false: {}", is_false);
    println!("Equal: {}", equal);
    println!("Not equal: {}", not_equal);
    println!("Greater than: {}", greater_than);
    println!("Less than: {}", less_than);
    println!("Less than or equal: {}", less_than_or_equal);

    // Logical expressions
    let and_result: bool = is_true && !is_false;
    let or_result: bool = is_true || is_false;
    let not_result: bool = !is_true;

    println!("AND result: {}", and_result);
    println!("OR result: {}", or_result);
    println!("NOT result: {}", not_result);

    // String expressions
    let greeting = "Hello, ";
    let name = "Rust!";
    // I'm not 100% sure why but when adding strings together we need to use the .to_string() method.
    let combined_string = greeting.to_string() + name;

    println!("Combined String: {}", combined_string);
}
