use crate::guards::Token;
use rocket::get;

#[get("/")]
pub(super) fn test(token: Token) -> String {
    token.0
}
