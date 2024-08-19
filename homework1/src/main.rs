use std::io::Write;

enum Operation {
    Add { x: f64, y: f64 },
    Subtract { x: f64, y: f64 },
    Multiply { x: f64, y: f64 },
    Divide { x: f64, y: f64 }
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add { x, y } => x + y,
        Operation::Subtract { x, y } => x - y,
        Operation::Multiply { x, y } => x * y,
        Operation::Divide { x, y } => x / y
    }
}

fn prompt(show: &str) -> Option<String> {
    let stdin = std::io::stdin();
    let mut input = String::new();

    print!("{}", show);
    std::io::stdout().flush().expect("Stdout flushing failed");

    match stdin.read_line(&mut input) {
        Ok(_) => Some(input.trim().to_string()),
        Err(_) => None,
    }
}

fn main() {
    let first_number = prompt("First number: ").unwrap();
    let operation = prompt("Operation: ").unwrap();
    let second_number = prompt("Second number: ").unwrap();

    let parsed_first_res = first_number.parse::<f64>();
    let parsed_second_res = second_number.parse::<f64>();
    
    if let (Ok(number1), Ok(number2)) = (parsed_first_res, parsed_second_res) {
        let operation_result: f64 = match operation.as_str() {
            "+" => calculate(Operation::Add { x: number1, y: number2 }),
            "-" => calculate(Operation::Subtract { x: number1, y: number2 }),
            "*" => calculate(Operation::Multiply { x: number1, y: number2 }),
            "/" => calculate(Operation::Divide { x: number1, y: number2 }),
            _ => {
                eprintln!("Operation '{}' is not supported thus the result is -1.0", operation);
                -1.0
            }
        };

        println!("{} {} {} = {}", number1, operation, number2, operation_result);
    } else {
        eprintln!("One of the numbers couldn't be parsed!");
    }
}
