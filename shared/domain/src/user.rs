use nutype::nutype;
use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::UserFacingError;

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

#[nutype(validate(max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DisplayName(String);

impl DisplayName {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for DisplayNameError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::TooLong => "Display name must be at most 30 characters",
        }
    }
}

static EMAIL_REGEX: OnceCell<EmailRegex> = OnceCell::new();

#[derive(Debug)]
pub struct EmailRegex(Regex);

impl EmailRegex {
    pub fn global() -> &'static Self {
        EMAIL_REGEX.get().expect("email regex is not initialized")
    }

    pub fn init() -> Self {
        Self(regex::Regex::new(r#"^\S+@\S+\.\S{1,64}$"#).unwrap())
    }

    pub fn is_valid<T: AsRef<str>>(&self, email: T) -> bool {
        self.0.is_match(email.as_ref())
    }
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = EMAIL_REGEX.get_or_init(EmailRegex::init);

    email_regex.is_valid(email)
}

#[nutype(validate(with = is_valid_email))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Email(String);

// NOTE The nutype validate macro automatically generates the EmailError enum
impl UserFacingError for EmailError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Invalid => "Email is not valid. Format: your_name@example.com",
        }
    }
}
