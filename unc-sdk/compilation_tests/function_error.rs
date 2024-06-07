//! Testing FunctionError macro.

use borsh::BorshSerialize;
use unc_sdk::FunctionError;
use unc_sdk::unc;
use std::fmt;

#[derive(FunctionError, BorshSerialize)]
struct ErrorStruct {
    message: String,
}

impl fmt::Display for ErrorStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error occurred: {}", self.message)
    }
}

#[derive(FunctionError, BorshSerialize)]
enum ErrorEnum {
    NotFound,
    Banned { account_id: String },
}

impl fmt::Display for ErrorEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorEnum::NotFound => write!(f, "not found"),
            ErrorEnum::Banned { account_id } => write!(f, "account {} is banned", account_id),
        }
    }
}

#[derive(Default)]
#[unc(contract_state)]
struct Contract {}

#[unc]
impl Contract {
    #[handle_result]
    pub fn set(&self, value: String) -> Result<String, ErrorStruct> {
        Err(ErrorStruct { message: format!("Could not set to {}", value) })
    }

    #[handle_result]
    pub fn get(&self) -> Result<String, ErrorEnum> {
        Err(ErrorEnum::NotFound)
    }
}

fn main() {}
