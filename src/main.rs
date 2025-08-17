use std::io;
use std::ops::Sub;

enum CalcError{
    DivideByZero,
}


struct Add;
struct Subtract;
struct BitShiftLeft;
struct BitShiftRight;

trait Calculation {
    fn process(&self, x: i64, y: i64) -> i64;
}

impl Calculation for Add {
    fn process(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

impl Calculation for Subtract {
    fn process(&self, x: i64, y: i64) -> i64 {
        x - y
    }
}

impl Calculation for BitShiftLeft {
    fn process(&self, x: i64, y: i64) -> i64 {
        x << y
    }
}

impl Calculation for BitShiftRight {
    fn process(&self, x: i64, y: i64) -> i64 {
        x >> y
    }
}

fn main() {
    println!("Cast does Countdown application!");

    let add: Box<dyn Calculation> = Box::new(Add);
    let subtract: Box<dyn Calculation> = Box::new(Subtract);
    let bs_left: Box<dyn Calculation> = Box::new(BitShiftLeft);
    let bs_right: Box<dyn Calculation> = Box::new(BitShiftRight);

    println!("Here is the sum: {:?}", add.process(3, 2));
    println!("Here is the subtract: {:?}", subtract.process(3, 2));
    println!("Here is the bitshift left: {:?}", bs_left.process(3, 2));
    println!("Here is the bitshift right: {:?}", bs_right.process(9, 1));

    // Input the countdown numbers
    println!("Please enter a comma seperated list of numbers (e.g. \"1,2,3,50,75\")");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let numbers: Vec<i16> = input_line
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("Please enter only integers (separated by spaces or commas)");


    println!("You entered: {:?}", numbers);
}
