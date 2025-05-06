use std::io::{self, Write};

#[derive(Debug, thiserror::Error)]
pub enum CalculatorError {
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}

struct Calculator {
    result: f64,
}

impl Calculator {
    fn new() -> Self {
        Calculator { result: 0.0 }
    }

    fn add(&mut self, x: f64) -> Result<(), CalculatorError> {
        self.result += x;
        Ok(())
    }

    fn subtract(&mut self, x: f64) -> Result<(), CalculatorError> {
        self.result -= x;
        Ok(())
    }

    fn multiply(&mut self, x: f64) -> Result<(), CalculatorError> {
        self.result *= x;
        Ok(())
    }

    fn divide(&mut self, x: f64) -> Result<(), CalculatorError> {
        if x == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }
        self.result /= x;
        Ok(())
    }

    fn clear(&mut self) {
        self.result = 0.0;
    }

    fn get_result(&self) -> f64 {
        self.result
    }
}

fn print_menu() {
    println!("\nCalculator Menu:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("5. Clear");
    println!("6. Show Result");
    println!("7. Exit");
    print!("Enter your choice (1-7): ");
    io::stdout().flush().unwrap();
}

fn get_number() -> Result<f64, CalculatorError> {
    print!("Enter a number: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse::<f64>().map_err(|_| {
        CalculatorError::InvalidOperation("Invalid number format".to_string())
    })
}

fn main() -> Result<(), CalculatorError> {
    let mut calculator = Calculator::new();
    println!("Welcome to the Calculator!");
    println!("Current result: {}", calculator.get_result());

    loop {
        print_menu();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        
        match choice.trim() {
            "1" => {
                let num = get_number()?;
                calculator.add(num)?;
                println!("Added {}. Current result: {}", num, calculator.get_result());
            }
            "2" => {
                let num = get_number()?;
                calculator.subtract(num)?;
                println!("Subtracted {}. Current result: {}", num, calculator.get_result());
            }
            "3" => {
                let num = get_number()?;
                calculator.multiply(num)?;
                println!("Multiplied by {}. Current result: {}", num, calculator.get_result());
            }
            "4" => {
                let num = get_number()?;
                match calculator.divide(num) {
                    Ok(_) => println!("Divided by {}. Current result: {}", num, calculator.get_result()),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "5" => {
                calculator.clear();
                println!("Calculator cleared. Current result: {}", calculator.get_result());
            }
            "6" => {
                println!("Current result: {}", calculator.get_result());
            }
            "7" => {
                println!("Thank you for using the Calculator!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}
