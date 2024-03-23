use crate::errors::AobaError;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct Token(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = AobaError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get("Authorization").next();
        match auth_header {
            None => Outcome::Error((Status::new(401), AobaError::Unauthorized)),
            Some(token) => {
                let token = token.split(' ').nth(1);
                match token {
                    Some(token) => Outcome::Success(Token(token.to_owned())),
                    None => Outcome::Error((Status::new(401), AobaError::Unauthorized)),
                }
            }
        }
    }
}
