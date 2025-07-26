// filepath: loan-approval-system/src/utils.rs
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum InputError {
    InvalidAge,
    InvalidIncome,
    ParseError(ParseFloatError),
}

pub fn validate_age(age: &str) -> Result<u32, InputError> {
    match age.parse::<u32>() {
        Ok(value) if value >= 18 => Ok(value),
        Ok(_) => Err(InputError::InvalidAge),
        Err(_) => Err(InputError::InvalidAge),
    }
}

pub fn validate_income(income: &str) -> Result<f64, InputError> {
    match income.parse::<f64>() {
        Ok(value) if value > 0.0 => Ok(value),
        Ok(_) => Err(InputError::InvalidIncome),
        Err(err) => Err(InputError::ParseError(err)),
    }
}