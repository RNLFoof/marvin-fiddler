use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum MarvinError {
    #[error("Failed to actually send the request.")]
    RequestSend(#[source] reqwest::Error),
    #[error("Bad request, maybe your credentials are wrong? Check src/secrets.rs")]
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
