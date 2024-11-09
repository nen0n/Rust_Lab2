use std::io::{self, Write};

struct Calculator {
    memory: f64,
    current: f64,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {
            memory: 0.0,
            current: 0.0,
        }
    }

    fn basic_operation(&mut self, op: char, num: f64) -> Result<f64, &'static str> {
        self.current = match op {
            '+' => self.current + num,
            '-' => self.current - num,
            '*' => self.current * num,
            '/' if num != 0.0 => self.current / num,
            '/' => return Err("Division by zero"),
            _ => return Err("Invalid operator"),
        };
        Ok(self.current)
    }

    fn store_memory(&mut self) {
        self.memory = self.current;
    }

    fn recall_memory(&mut self) {
        self.current = self.memory;
    }
}

fn evaluate_rpn(expr: &str) -> Result<f64, &'static str> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {
                    return Err("Invalid expression");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" if b != 0.0 => a / b,
                    "/" => return Err("Division by zero"),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num => match num.parse::<f64>() {
                Ok(n) => stack.push(n),
                Err(_) => return Err("Invalid number"),
            },
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression");
    }
    Ok(stack[0])
}

fn main() {
    let mut calc = Calculator::new();

    loop {
        print!("\nSelect mode (1: Basic Calculator, 2: RPN Calculator, 3: Exit): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Basic Calculator Mode");
                println!("Current value: {}", calc.current);
                println!("Available commands:");
                println!("  number: Set current value");
                println!("  +/-/*/_ number: Perform operation");
                println!("  m: Store to memory");
                println!("  r: Recall from memory");
                println!("  q: Return to main menu");

                loop {
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if input == "q" {
                        break;
                    } else if input == "m" {
                        calc.store_memory();
                        println!("Stored {} in memory", calc.current);
                    } else if input == "r" {
                        calc.recall_memory();
                        println!("Recalled {}", calc.current);
                    } else {
                        let parts: Vec<&str> = input.split_whitespace().collect();
                        match parts.as_slice() {
                            [num] => {
                                match num.parse::<f64>() {
                                    Ok(n) => {
                                        calc.current = n;
                                        println!("Current value: {}", calc.current);
                                    }
                                    Err(_) => println!("Invalid number"),
                                }
                            }
                            [op, num] => {
                                match num.parse::<f64>() {
                                    Ok(n) => {
                                        match calc.basic_operation(op.chars().next().unwrap(), n) {
                                            Ok(result) => println!("Result: {}", result),
                                            Err(e) => println!("Error: {}", e),
                                        }
                                    }
                                    Err(_) => println!("Invalid number"),
                                }
                            }
                            _ => println!("Invalid input"),
                        }
                    }
                }
            }
            "2" => {
                println!("RPN Calculator Mode");
                println!("Enter expression in RPN format (e.g., '3 4 + 5 *')");
                println!("Enter 'q' to return to main menu");

                loop {
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if input == "q" {
                        break;
                    }

                    match evaluate_rpn(input) {
                        Ok(result) => println!("Result: {}", result),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
