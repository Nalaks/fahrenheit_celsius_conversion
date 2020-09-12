use std::io;

fn main() {
    println!("Fahrenheit to Celsius or backwards converter.");
    println!("Please input 'f' for conversion to Fahrenheit or 'c' for conversion to celsius: ");

    let mut choice = String::new();
    let mut num_to_convert = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read the line");
    let choice = choice.trim().to_lowercase();

    println!("Please input number to convert: ");

    io::stdin()
        .read_line(&mut num_to_convert)
        .expect("Failed to read the line");
    let num_to_convert: i32 = num_to_convert.trim().parse().expect("failed to convert");

    let fahrenheit_conversion = (num_to_convert * 9 / 5) + 32;
    let celsius_conversion = (num_to_convert - 32) * 5 / 9;

    if choice == "f" {
        println!("{}°F", fahrenheit_conversion);
    } else if choice == "c" {
        println!("{}°C", celsius_conversion);
    } else {
        println!("Please type 'f' or 'c'.");
    }
}
