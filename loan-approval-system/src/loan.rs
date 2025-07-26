use std::fmt;

#[derive(Debug)]
pub enum AgeError {
    TooYoung,
    Invalid,
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgeError::TooYoung => write!(f, "Applicant is too young"),
            AgeError::Invalid => write!(f, "Invalid age"),
        }
    }
}
impl std::error::Error for AgeError {}

#[derive(Debug)]
pub enum IncomeError {
    TooLow,
    Invalid,
}

impl fmt::Display for IncomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IncomeError::TooLow => write!(f, "Income too low"),
            IncomeError::Invalid => write!(f, "Invalid income"),
        }
    }
}
impl std::error::Error for IncomeError {}

pub struct LoanApplication {
    pub income: f64,
    pub age: u32,
    pub loan_amount: f64,
    pub co_applicant_income: Option<f64>,
}

impl LoanApplication {
    pub fn new(income: f64, age: u32, loan_amount: f64, co_applicant_income: Option<f64>) -> Self {
        LoanApplication {
            income,
            age,
            loan_amount,
            co_applicant_income,
        }
    }

    fn check_age(&self) -> Result<(), AgeError> {
        if self.age < 18 {
            Err(AgeError::TooYoung)
        } else {
            Ok(())
        }
    }

    fn check_income(&self) -> Result<(), IncomeError> {
        let total_income = self.income + self.co_applicant_income.unwrap_or(0.0);
        if total_income < 20000.0 {
            Err(IncomeError::TooLow)
        } else {
            Ok(())
        }
    }

    pub fn check_eligibility(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.check_age()?;
        self.check_income()?;
        Ok(())
    }
}