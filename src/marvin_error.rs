use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum MarvinError {
    #[error("Failed to read the file.")]
    RequestSend(#[source] reqwest::Error),
    // #[display("Wow, it's an error code with the gggggggggggfollowing integer value: {_0}")]
    #[error("Failed to read the file.")]
    BadRequest(#[source] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marvin_error() {
        let client = reqwest::blocking::Client::new();
        let error = || {
            client
                .post(format!("invalid url"))
                .send()
                .expect_err("WHAT??")
        };

        MarvinError::BadRequest(error());
        MarvinError::RequestSend(error());

        assert!(matches!(
            MarvinError::BadRequest(error()),
            MarvinError::BadRequest(_),
        ));

        assert!(!matches!(
            MarvinError::BadRequest(error()),
            MarvinError::RequestSend(_),
        ));
    }
}
