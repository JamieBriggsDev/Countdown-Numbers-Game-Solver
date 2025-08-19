mod ops;
mod input;
mod solution;

use std::time::Instant;

fn main() {
    println!("Cast does Countdown application!");

    let numbers = input::get_input_numbers();
    println!("Using numbers: {:?}", numbers);

    let target = input::get_target_number();

    // Start counting time after getting numbers, the user takes a few seconds to input numbers
    //  which would lead to an unrealistic stopwatch timer.
    let start = Instant::now();


    solution::run(&numbers,target);
    

    let elapsed = start.elapsed();
    println!("Completed in {:?}ms", elapsed.as_millis());
}
