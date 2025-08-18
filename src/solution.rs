// solver.rs
use crate::ops::Calculation;

pub fn run(numbers: &[i64], operations: &[Box<dyn Calculation>]) {
    // Go through every type of equation that could be called
    let mut count: i64 = 0;

    // Checks permutations involving just two numbers
    'outer: for x_idx in 0..numbers.len() {
        'inner: for y_idx in 0..numbers.len() {
            let x = numbers[x_idx];
            let y = numbers[y_idx];
            // Skip if it's the same as inner
            if x == y {
                break 'inner;
            }
            'operation: for opp in operations {
                match opp.process(x, y) {
                    Ok(result) => {
                        count = count + 1;
                        println!("Performing: {x} {} {y} = {:?}", opp.symbol(), result);
                    }
                    Err(_e) => {
                        continue 'operation;
                    }
                }
            }
        }
    }

    println!("You found {count} combinations!");

}