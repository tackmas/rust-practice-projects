use std::io;

fn main() {
    println!("Which Fibonacci number do you want to find?");

    let fibonacci_index: u64 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong! (1)");

        break match input.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                println!("You have entered a invalid input. Please write a valid positive integer.");
                continue
            }
        };
    };

    fibonacci_number(fibonacci_index);
}

fn fibonacci_number(f_index: u64) {
    if f_index == 0 {
        println!("Fibonacci number 0 is equal to 0.");
        return;
    } else if f_index == 1 {
        println!("Fibonacci number 1 is equal to 1.");
        return;
    };

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = 0;

    for _ in 2..=f_index {
        c = a + b;
        a = b;
        b = c;
    };

    println!("Fibonacci number {f_index} is equal to {c}.");
}




/*

Ignore:

The nth Fibonacci number can be computed by Binet's formula: F = ([φ]^n - [ψ]^n) / sqrt[5]) 
where φ is the golden ratio ([1/2] * [1 + sqrt{5}]) and ψ is its conjugate ([1/2] * [1 - sqrt{5}]).

Binet's formula can, with the help of some math, be transformed to: 
F = (1/2^[n-1]) * (nC.1 * 5^[{1-1}/2] + nC.3 * 5^[{3-1}/2] + nC.5 * 5^[{5-1}/2] + ... + nC.[n-1] * 5^[{n-1}/2])
where nC.[r] is the combination notation, n is the number of items in the set, and r is the number of items to choose.

*/
