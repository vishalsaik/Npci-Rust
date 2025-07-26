pub struct ATM {
    total_cash: f64,
}

impl ATM {
    pub fn new(initial_cash: f64) -> Self {
        ATM {
            total_cash: initial_cash,
        }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be greater than zero.".to_string());
        }
        if amount > self.total_cash {
            return Err("Insufficient funds in the ATM.".to_string());
        }
        self.total_cash -= amount;
        Ok(amount)
    }

    pub fn get_total_cash(&self) -> f64 {
        self.total_cash
    }
}