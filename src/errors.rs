use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct InteractionError {
    pub message: String,
}

impl Display for InteractionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error handling interaction: {}", self.message)
    }
}
