use std::io;

fn main() {
    // intro
    println!("Fahrenheit and Celsius temperature converter");

    // loop forever
    loop {
        // input Fahrenheit temperature
        println!("Please enter a temperature in Fahrenheit:");
        let mut fahrenheit_value = String::new();
        io::stdin()
            .read_line(&mut fahrenheit_value)
            .expect("Failed to read line");
        let fahrenheit_value: f64 = match fahrenheit_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You entered {fahrenheit_value}");

        // calculate and output Celsius temperature
        let celsius_value = (fahrenheit_value - 32.0) * (5.0 / 9.0);
        println!("{fahrenheit_value}°F is equal to {celsius_value:.2}°C");
    }
}
