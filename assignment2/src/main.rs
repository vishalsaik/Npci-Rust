
mod loan;
mod utils;

use std::io::{self, Write};

fn main() {
    println!("Loan Approval System");

    let income = prompt_and_parse("Enter your income: ", utils::validate_income);
    let age = prompt_and_parse("Enter your age: ", utils::validate_age);
    let loan_amount = prompt_and_parse("Enter desired loan amount: ", utils::validate_income);

    let co_applicant_income = {
        print!("Do you have a co-applicant? (y/n): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim().eq_ignore_ascii_case("y") {
            Some(prompt_and_parse("Enter co-applicant income: ", utils::validate_income))
        } else {
            None
        }
    };

    let application = loan::LoanApplication::new(income, age, loan_amount, co_applicant_income);

    match application.check_eligibility() {
        Ok(_) => println!("Loan Approved!"),
        Err(e) => println!("Loan Denied: {}", e),
    }
}

fn prompt_and_parse<T, F>(prompt: &str, parser: F) -> T
where
    F: Fn(&str) -> Result<T, utils::InputError>,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match parser(input.trim()) {
            Ok(val) => return val,
            Err(e) => println!("Error: {:?}", e),
        }
    }
}