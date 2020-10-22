use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

fn validate_token(token: &str) -> Result<bool, Error> {
  // TODO: add validate method
  Ok(true)
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
  let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
  match validate_token(credentials.token()) {
    Ok(result) => {
      if result == true {
        Ok(req)
      } else {
        Err(AuthenticationError::from(config).into())
      }
    },
    Err(_) => Err(AuthenticationError::from(config).into()),
  }
}
