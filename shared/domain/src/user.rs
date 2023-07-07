use nutype::nutype;
use serde::{Deserialize, Serialize};

use crate::UserFacingError;

// TODO refactor error handling to improve reusability
// NOTE Using the `nutype` crate to easily add field validation
#[nutype(validate(present, min_len = 3, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Username(String);

// NOTE The nutype validate macro automatically generates the UsernameError enum
impl UserFacingError for UsernameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "User name cannot be empty",
            Self::TooShort => "User name must be at least 3 characters",
            Self::TooLong => "User name must be at most 30 characters",
        }
    }
}

#[nutype(validate(present, min_len = 8))]
#[derive(AsRef, Clone, Serialize, Deserialize, PartialEq)]
pub struct Password(String);

impl UserFacingError for PasswordError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Password cannot be empty",
            Self::TooShort => "Password must be at least 8 characters",
        }
    }
}
