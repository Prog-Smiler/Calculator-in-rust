use std::io;
use std::io::Write;

fn main() {
    let number1 = get_f32_input("Enter the first number: ");
    let number2 = get_f32_input("Enter the second number: ");

    println!("Addition: {}", number1 + number2);
    println!("Substraction: {}", number1 - number2);
    println!("Multiplication: {}", number1 * number2);
    if number2 != 0.0 {
        println!("Divison (quotient): {}", number1 / number2);
        println!("Divison (remainder): {}", number1 % number2);
    } else {
        println!("Not divisible by zero");
    }
}

//function for input and managing if user entered a very large number
fn get_f32_input(prompt: &str) -> f32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Flush failed");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Ok(num) = input.trim().parse::<f32>() {
            if num.is_infinite() {
                println!("Error: Num is too large! Max is ~3.4e38.");
            } else {
                return num;
            }
        } else {
            println!("Enter a valid number! ");
        }
    }
}
