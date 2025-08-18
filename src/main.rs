use std::fmt;
use std::io;
use std::time::Instant;

struct Add;
struct Subtract;
struct BitShiftLeft;
struct BitShiftRight;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalcError {
    NegativeShift,
    ShiftOutOfRange,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::NegativeShift => write!(f, "shift count must be non-negative"),
            CalcError::ShiftOutOfRange => write!(f, "shift count out of range for i64"),
        }
    }
}

trait Calculation {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError>;
    fn symbol(&self) -> &str;
}

impl Calculation for Add {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        Ok(x + y)
    }
    fn symbol(&self) -> &str {
        "+"
    }
}

impl Calculation for Subtract {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        Ok(x - y)
    }

    fn symbol(&self) -> &str {
        "-"
    }
}

impl Calculation for BitShiftLeft {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        let shift: u32 = y.try_into().map_err(|_| CalcError::NegativeShift)?;
        x.checked_shl(shift).ok_or(CalcError::ShiftOutOfRange)
    }

    fn symbol(&self) -> &str {
        "<<"
    }
}

impl Calculation for BitShiftRight {
    fn process(&self, x: i64, y: i64) -> Result<i64, CalcError> {
        let shift: u32 = y.try_into().map_err(|_| CalcError::NegativeShift)?;
        x.checked_shr(shift).ok_or(CalcError::ShiftOutOfRange)
    }

    fn symbol(&self) -> &str {
        ">>"
    }
}

fn main() {
    println!("Cast does Countdown application!");

    let numbers = get_input_numbers();
    println!("Using numbers: {:?}", numbers);

    // Start counting time after getting numbers, the user takes a few seconds to input numbers
    //  which would lead to an unrealistic stopwatch timer.
    let start = Instant::now();

    let mut operations: Vec<Box<dyn Calculation>> = Vec::new();

    operations.push(Box::new(Add));
    operations.push(Box::new(Subtract));
    operations.push(Box::new(BitShiftLeft));
    operations.push(Box::new(BitShiftRight));

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
            'operation: for opp in &operations {
                match opp.process(x, y) {
                    Ok(result) =>{
                        count = count + 1;
                        println!(
                            "Performing: {x} {} {y} = {:?}",
                            opp.symbol(),
                            result
                        );
                    }
                    Err(_e) => {
                        continue 'operation;
                    }
                }
            }
        }
    }

    println!("You found {count} combinations!");

    let elapsed = start.elapsed();
    println!("Completed in {:?}ms", elapsed.as_millis());
}

#[cfg(not(debug_assertions))]
fn get_input_numbers() -> Vec<i64> {
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
fn get_input_numbers() -> Vec<i64> {
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
