use rocket::post;
use crate::guards::Token;

#[post("/")]
pub(super) async fn upload_image(token: Token) -> &'static str {
    let Token(token) = token;
    "Hello, world!"
}