use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dog {
    status: String,
    message: String
}

impl Display for Dog {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "message: {}, status: {}", self.message, self.status)
    }
}