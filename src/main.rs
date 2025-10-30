// Fahrenheit to Celsius => C = (5.0/9.0) * (F - 32.0)  
// Celsius to Fahrenheit => F = (9.0/5.0) * C + 32.0
// [C is Celsius and F is Fahrenheit]

const CELSIUS_ABSOLUTE_ZERO: f64 = -273.15;
const FAHRENHEIT_ABSOLUTE_ZERO: f64 = -459.67;

// didnt know the max value was so large
const CELSIUS_MAX_F64: f64 = (5.0/9.0) * (std::f64::MAX - 32.0); /* For large postive numbers, Celsius converted to Fahrenheit yields a greater number. 
                                                                    This could go above the max floating point value after the conversion. 
                                                                    Therefore we set a lower max for Celsius, if converted to Fahrenheit equals std::f64::MAX and not higher.
                                                                */
const FAHRENHEIT_MAX_F64: f64 = std::f64::MAX;

use std::io;

fn main() {
    println!("Hello! Do you want to convert Fahrenheit to Celsius (1), or Celsius to Fahrenheit (2). 
    Input the number ('1' or '2') corresponding to the conversion you request:");
    
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Somthing went wrong! (1)");

        match input.trim().parse() {
            Ok(1) => break fahrenheit_to_celsius(),
            Ok(2) => break celsius_to_fahrenheit(),
            Ok(_) | Err(_) => {
                println!("Please input '1' or '2':");
                continue;
            }
        };
    };
}

fn fahrenheit_to_celsius() {
    println!("Input the value in Fahrenheit you wish to convert to Celsius:");

    let fahrenheit: f64 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong! (2)");

        break match input.trim().parse() {
            Ok(fahrenheit) => {
                if fahrenheit < FAHRENHEIT_ABSOLUTE_ZERO {
                    println!("That's below absolute zero! Please input a valid temperature:");
                    continue
                } else if fahrenheit > FAHRENHEIT_MAX_F64 {
                    println!("Too great of a number! The computer can't hold its greatness! Please enter a 'lesser number' that the computer can behold.");
                    continue;
                } else {
                    fahrenheit
                }
            }
            Err(_) => {
                println!("Please input a valid integer:");
                continue;
            }    
        };
    };

    let celsius = (5.0/9.0) * (fahrenheit - 32.0); 

    println!("Your value in Celsius is: {celsius} °C");
}

fn celsius_to_fahrenheit() {
    println!("Input the value in Celsius you wish to convert to Fahrenheit:");

    let celsius: f64 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong! (2)");

        break match input.trim().parse() {
            Ok(celsius) => {
                if celsius < CELSIUS_ABSOLUTE_ZERO {
                    println!("That's below absolute zero! Please input a valid temperature:");
                    continue
                } else if celsius > CELSIUS_MAX_F64 {
                    println!("Too great of a number! The computer can't hold its greatness! Please enter a 'lesser number' that the computer can behold.");
                    continue
                } else {
                    celsius
                }
            }
            Err(_) => {
                println!("Please input a valid temperature:");
                continue
            }    
        };
    };

    let fahrenheit = (9.0/5.0) * celsius + 32.0; 

    println!("Your value in Fahrenheit is: {fahrenheit} °F");
}
