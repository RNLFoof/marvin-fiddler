use std::{error::Error, fmt};
use derive_more::From;

#[derive(From)]
pub enum MarvinError {
    // #[display("Wow, it's an error code with the following integer value: {_0}")]
    RequestSend(Box<dyn Error>),
    // #[display("Wow, it's an error code with the gggggggggggfollowing integer value: {_0}")]
    BadRequest(u64),
}

impl PartialEq for MarvinError {
    // Required method
    fn eq(&self, other: &MarvinError) -> bool {
        self == other
    }

    // Provided method
    fn ne(&self, other: &MarvinError) -> bool {
        self != other
    }
}
	
// Implement std::fmt::Display for AppError
impl fmt::Display for MarvinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for MarvinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl std::error::Error for MarvinError {}

#[cfg(test)]
mod tests {
    use std::io::{Error, ErrorKind};

    use super::*;
    
    #[test]
    fn test_marvin_error() {
        assert_eq!(
            MarvinError::BadRequest(6), 
            MarvinError::RequestSend(
                Box::new(Error::new(ErrorKind::Other, "oh no!"))
            )
        )
    }
}
