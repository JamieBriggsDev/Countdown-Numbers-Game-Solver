use crate::ops;
// solver.rs
use crate::ops::Calculation;

// Make Expression cloneable
#[derive(Clone)]
pub struct Expression {
    pub value: i64,
    pub repr: String,
}

pub fn run(numbers: &[i64], target: i64) {
    let expressions: Vec<Expression> = numbers
        .iter()
        .map(|&n| Expression {
            value: n,
            repr: n.to_string(),
        })
        .collect();
    let operations = ops::default_operations();

    // Start recursive search
    search(expressions, target, &operations);
}

fn search(numbers: Vec<Expression>, target: i64, operations: &[Box<dyn Calculation>]) {
    // If any of the numbers are the result, return just that number
    for expr in &numbers {
        if expr.value == target {
            println!("Found solution: {} = {}", expr.repr, target);
            return;
        }
    }

    if numbers.len() == 1 {
        // No solution found in a recursive branch
        return;
    }

    // Checks permutations involving just two numbers
    for x_idx in 0..numbers.len() {
        'inner: for y_idx in 0..numbers.len() {
            let x = &numbers[x_idx];
            let y = &numbers[y_idx];
            // Skip if it's the same as inner
            if x_idx == y_idx {
                break 'inner;
            }
            for opp in operations {
                if let Ok(result) = opp.process(x.value, y.value) {
                    let new_expr = Expression {
                        value: result,
                        repr: format!("({} {} {})", x.repr, opp.symbol(), y.repr),
                    };

                    // Create new list and call search recursively
                    let mut next_numbers: Vec<Expression> = Vec::new();
                    for (k, expr) in numbers.iter().enumerate() {
                        if k != x_idx && k != y_idx {
                            next_numbers.push(expr.clone());
                        }
                    }

                    next_numbers.push(new_expr);

                    // Call search recusively
                    search(next_numbers, target, operations);
                }
            }
        }
    }

    // No solutions found
}
