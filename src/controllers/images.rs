use crate::errors::AobaError;
use crate::guards::Token;
use crate::services::image_service::ImageService;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::{post, FromForm, State};

#[derive(FromForm)]
pub(super) struct ImageUploadForm<'a> {
    image: TempFile<'a>,
}

#[post("/", data = "<data>")]
pub(super) async fn upload_image(
    token: Token,
    image_service: &State<ImageService>,
    mut data: Form<ImageUploadForm<'_>>,
) -> Result<String, AobaError> {
    let Token(token) = token;
    let filename = image_service.upload_image(&mut data.image, token).await?;
    Ok(filename)
}
