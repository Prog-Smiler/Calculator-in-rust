use std::io;

fn main() {
    let mut input = String::new();

    print!("Enter the first number: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number1: f32 = input.trim().parse().expect("Enter a valid number!");
    input.clear();

    print!("Enter the second number: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number2: f32 = input.trim().parse().expect("Enter a valid number!");

    println!("Addition: {}", number1 + number2);
    println!("Substraction: {}", number1 - number2);
    println!("Multiplication: {}", number1 * number2);
    println!("Divison (quotient): {}", number1 / number2);
    println!("Divison (remainder): {}", number1 % number2);
}
