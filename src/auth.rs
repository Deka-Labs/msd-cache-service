use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};

use crate::login_service::LoginService;

/// Contains auth information from request
#[derive(Debug)]
pub struct AuthInfo {
    pub user_id: i32,
}

/// All errors getting auth info
#[derive(Debug)]
pub enum AuthError {
    /// Failed to find required Authetication header
    NoHeader,
    /// Specified auth method in header is not supported
    NotSupportedAuth,
    /// Too many headers provided
    BadCount,
    /// Invalid header format
    HeaderFormatInvalid,
    /// Invalid credentials
    InvalidCredentials,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthInfo {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auths_headers: Vec<_> = req.headers().get("Authorization").collect();

        // Parsing
        if auths_headers.is_empty() {
            return Outcome::Failure((Status::Unauthorized, AuthError::NoHeader));
        }

        if 1 < auths_headers.len() {
            return Outcome::Failure((Status::BadRequest, AuthError::BadCount));
        }

        let mut auth_str = auths_headers[0].split_whitespace();
        let Some(scheme) = auth_str.next() else {
            return Outcome::Failure((Status::BadRequest, AuthError::HeaderFormatInvalid));
        };

        if scheme != "Basic" {
            return Outcome::Failure((Status::NotImplemented, AuthError::NotSupportedAuth));
        }

        let Some(auth_data) = auth_str.next() else {
            return Outcome::Failure((Status::BadRequest, AuthError::HeaderFormatInvalid));
        };

        // Decode base64
        let Ok(credentials_raw) = base64::decode(auth_data) else {
            return Outcome::Failure((Status::BadRequest, AuthError::HeaderFormatInvalid));
        };

        let Ok(credentials) = String::from_utf8(credentials_raw) else {
            return Outcome::Failure((Status::BadRequest, AuthError::HeaderFormatInvalid));
        };

        let Some((email, password)) = credentials.split_once(':') else {
            return Outcome::Failure((Status::BadRequest, AuthError::HeaderFormatInvalid));
        };

        // Request from login_service correctness and get user id
        let login_service: &State<LoginService> = req
            .guard()
            .await
            .expect("Login service must be added to rocket");
        let Some(user_id) = login_service.login(email, password).await else {
            return Outcome::Failure((Status::Unauthorized, AuthError::InvalidCredentials));
        };

        Outcome::Success(AuthInfo { user_id })
    }
}
