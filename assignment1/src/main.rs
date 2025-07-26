// This is the entry point of the ATM simulator application.
mod atm;
mod account;
mod utils;

use atm::ATM;
use account::Account;
use std::io;

fn main() {
    let mut atm = ATM::new(1000.0); // Initialize ATM with $1000 cash
    let mut account = Account::new(500.0); // Initialize account with $500 balance

    loop {
        println!("Welcome to the ATM!");
        println!("1. Check Balance");
        println!("2. Withdraw Cash");
        println!("3. Exit");
        println!("Please select an option:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        match choice {
            1 => {
                println!("Your balance is: ${:.2}", account.check_balance());
            }
           2 => {
                println!("Enter amount to withdraw:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64 = amount.trim().parse().expect("Please enter a valid number");

                match atm.withdraw(amount) {
                    Ok(_) => {
                        account.deposit(-amount);
                        println!("Withdrawal successful! Please take your cash.");
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            3 => {
                println!("Thank you for using the ATM. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}