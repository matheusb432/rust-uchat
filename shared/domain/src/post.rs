use nutype::nutype;
use serde::{Deserialize, Serialize};

use crate::UserFacingError;

#[nutype(validate(present, max_len = 30))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Headline(String);

impl Headline {
    pub const MAX_CHARS: usize = 30;
}

impl UserFacingError for HeadlineError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Headline cannot be empty",
            Self::TooLong => "Headline must be at most 30 characters",
        }
    }
}

#[nutype(validate(present, max_len = 100))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Message(String);

impl Message {
    pub const MAX_CHARS: usize = 100;
}

impl UserFacingError for MessageError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Message cannot be empty",
            Self::TooLong => "Message must be at most 100 characters",
        }
    }
}

#[nutype(validate(present, max_len = 60))]
#[derive(AsRef, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Caption(String);

impl Caption {
    pub const MAX_CHARS: usize = 60;
}

impl UserFacingError for CaptionError {
    fn formatted_error(&self) -> &'static str {
        match self {
            Self::Missing => "Caption cannot be empty",
            Self::TooLong => "Caption must be at most 60 characters",
        }
    }
}
