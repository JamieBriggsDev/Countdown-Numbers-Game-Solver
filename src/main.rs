mod ops;
mod input;
mod solution;

use std::time::Instant;

fn main() {
    println!("Cast does Countdown application!");

    let numbers = input::get_input_numbers();
    println!("Using numbers: {:?}", numbers);

    // Start counting time after getting numbers, the user takes a few seconds to input numbers
    //  which would lead to an unrealistic stopwatch timer.
    let start = Instant::now();

    let operations = ops::default_operations();
    
    solution::run(&numbers, &operations);
    

    let elapsed = start.elapsed();
    println!("Completed in {:?}ms", elapsed.as_millis());
}
