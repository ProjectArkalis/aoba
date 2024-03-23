use rocket::get;
use crate::guards::Token;

#[get("/")]
pub(super) fn test(token: Token) -> String {
    token.0
}