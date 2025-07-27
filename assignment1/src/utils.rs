
pub fn validate_amount(amount: f64) -> Result<f64, String> {
    if amount <= 0.0 {
        Err("Amount must be greater than zero.".to_string())
    } else {
        Ok(amount)
    }
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn prompt_user(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}