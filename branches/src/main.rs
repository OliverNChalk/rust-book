use std::io;

fn main() {
    loop {
        println!("Choose an option:");
        println!("  1. Fahrenheit to Celsius");
        println!("  2. Generate nth Fibonacci number");
        println!("  3. Print Twelve Days of Christmas");

        let user_input = wait_user_uint();
        match user_input {
            1 => break fahrenheit(),
            2 => break fibonacci(),
            3 => break twelve_days(),
            _ => continue,
        }
    }
}

fn wait_user_uint() -> u32 {
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        return match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

fn wait_user_float() -> f64 {
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        return match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

fn fahrenheit() {
    println!("Enter your temperature in Fahrenheit:");

    let fahrenheit = wait_user_float();
    let celsius = (fahrenheit - 32.0) / 1.8;

    println!("{} fahrenheit is {} in celsius", fahrenheit, celsius);
}

fn fibonacci() {
    println!("What number in the Fibonacci sequence would you like?");

    let fib_index = wait_user_uint();
    let nth_fibonacci = calc_nth_fibonacci(fib_index);

    println!("The {}th Fibonacci number is {}", fib_index, nth_fibonacci);
}

fn calc_nth_fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 0..(n - 2) {
        let temp = current;

        current = current + previous;
        previous = temp;
    }

    return current;
}

fn twelve_days() {
    let verse_unique_line = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for n in 0..12 {
        verse_intro(n);

        for i in (0..=n).rev() {
            println!("{}", verse_unique_line[i]);
        }
    }
}

fn verse_intro(n: usize) {
    let number_to_word = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    println!(
        "On the {} day of Christmas, my true love sent to me",
        number_to_word[n],
    );
}
