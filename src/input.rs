// input.rs
use std::io;

#[cfg(not(debug_assertions))]
pub fn get_input_numbers() -> Vec<i64> {
    // Input the countdown numbers
    println!("Please enter a comma seperated list of numbers (e.g. \"1,2,3,50,75\")");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let numbers: Vec<i64> = input_line
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("Please enter only integers (separated by spaces or commas)");
    numbers
}

#[cfg(debug_assertions)]
pub fn get_input_numbers() -> Vec<i64> {
    // Return dev numbers
    let mut numbers: Vec<i64> = Vec::new();
    // 437
    // 75, 100, 5, 2, 2, and 4
    // Should be possible: 5 + ((100 - 75 + 2) << 4) = 437
    numbers.push(75);
    numbers.push(100);
    numbers.push(5);
    numbers.push(2);
    numbers.push(2);
    numbers.push(4);
    numbers
}

#[cfg(not(debug_assertions))]
pub fn get_target_number() -> i64 {
    // Input the target number
    println!("Please enter a target number");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    input_line
        .trim()
        .parse()
        .expect("Please enter a valid integer")
}

#[cfg(debug_assertions)]
pub fn get_target_number() -> i64 {
    437
}