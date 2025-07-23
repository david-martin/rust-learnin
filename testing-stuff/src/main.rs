fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(x / y)
    }
}

fn main() {
    println!("{}", add(3, 2));
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result is {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// Only include the below when running `cargo test`
#[cfg(test)]
// new module for our tests
mod tests {
    // import everything from parent
    use super::*;

    // Here's a test function, hence the #[test]
    // Better not panic!
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(10, 15), 25);
    }

    #[test]
    fn test_divide_success() {
        let result = divide(12.0, 3.0);
        assert_eq!(result, Ok(4.0))
    }
    
    #[test]
    fn test_divide_error() {
        let result = divide(12.0, 0.0);
        assert_eq!(result, Err("Cannot divide by zero".to_string()))
    }
}
