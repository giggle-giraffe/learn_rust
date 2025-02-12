// MAKE the code work by fixing all panics
fn main() -> Result<(), &'static str> {
    // This assertion is fine as it's a test case
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    let v = vec![1, 2, 3];
    
    // Instead of unwrap, use if let or match to handle the Option
    let ele = match v.get(2) {
        Some(value) => *value,
        None => return Err("Index out of bounds"),
    };

    // Handle potential overflow in production rate
    let v = match production_rate_per_hour(2) {
        Ok(rate) => rate,
        Err(e) => return Err(e),
    };

    // Handle division by zero
    if let Err(e) = divide(15, 1) {
        return Err(e);
    }

    println!("Success!");
    Ok(())
}

// Make divide return a Result to handle division by zero
fn divide(x: u8, y: u8) -> Result<(), &'static str> {
    if y == 0 {
        return Err("Division by zero!");
    }
    println!("{}", x / y);
    Ok(())
}

// Handle potential overflow in calculations
fn production_rate_per_hour(speed: u8) -> Result<f64, &'static str> {
    let cph: u8 = 21;
    
    // Check for potential overflow before multiplication
    let base_rate = match speed.checked_mul(cph) {
        Some(v) => v as f64,
        None => return Err("Speed calculation overflow"),
    };

    let rate = match speed {
        1..=4 => base_rate,
        5..=8 => base_rate * 0.9,
        9..=10 => base_rate * 0.77,
        _ => return Err("Invalid speed value"),
    };

    Ok(rate)
}

pub fn working_items_per_minute(speed: u8) -> Result<u32, &'static str> {
    let hourly_rate = production_rate_per_hour(speed)?;
    Ok((hourly_rate / 60.0) as u32)
}
