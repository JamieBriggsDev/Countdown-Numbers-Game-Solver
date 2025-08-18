use std::io;
struct Add;
struct Subtract;
struct BitShiftLeft;
struct BitShiftRight;

trait Calculation {
    fn process(&self, x: i64, y: i64) -> i64;
    fn symbol(&self) -> &str;
}

impl Calculation for Add {
    fn process(&self, x: i64, y: i64) -> i64 {
        x + y
    }
    fn symbol(&self) -> &str {
        "+"
    }
}

impl Calculation for Subtract {
    fn process(&self, x: i64, y: i64) -> i64 {
        x - y
    }

    fn symbol(&self) -> &str {
        "-"
    }
}

impl Calculation for BitShiftLeft {
    fn process(&self, x: i64, y: i64) -> i64 {
        // Convert to u32 and check bounds; perform a checked shift.
        let shift: u32 = match y.try_into() {
            Ok(s) => s,
            Err(_) => return 0, // negative shift count
        };
        x.checked_shl(shift).unwrap_or(0) // or: unwrap_or(x), or pick a policy
    }

    fn symbol(&self) -> &str {
        "<<"
    }
}

impl Calculation for BitShiftRight {
    fn process(&self, x: i64, y: i64) -> i64 {
        // Convert to u32 and check bounds; perform a checked shift.
        let shift: u32 = match y.try_into() {
            Ok(s) => s,
            Err(_) => return 0, // Negative shift count, return 0
        };
        x.checked_shr(shift).unwrap_or(0)

    }

    fn symbol(&self) -> &str {
        ">>"
    }
}

fn main() {
    println!("Cast does Countdown application!");

    let mut operations: Vec<Box<dyn Calculation>> = Vec::new();

    operations.push(Box::new(Add));
    operations.push(Box::new(Subtract));
    operations.push(Box::new(BitShiftLeft));
    operations.push(Box::new(BitShiftRight));

    let numbers = get_input_numbers();
    println!("Using numbers: {:?}", numbers);

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
                count = count + 1;
                println!(
                    "Performing: {x} {:?} {y} = {:?}",
                    opp.symbol(),
                    opp.process(x, y)
                );
            }
        }
    }

    println!("You found {count} combinations!")
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
