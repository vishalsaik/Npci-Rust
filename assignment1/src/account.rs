pub struct Account {
    balance: f64,
}

impl Account {
    pub fn new(initial_balance: f64) -> Self {
        Account {
            balance: initial_balance,
        }
    }

    pub fn check_balance(&self) -> f64 {
        self.balance
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount > self.balance {
            Err(String::from("Insufficient funds"))
        } else {
            self.balance -= amount;
            Ok(amount)
        }
    }
}