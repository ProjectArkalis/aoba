use rocket::{FromForm, post, State};
use rocket::form::Form;
use rocket::fs::TempFile;
use crate::errors::AobaError;
use crate::guards::Token;
use crate::services::image_service::ImageService;

#[derive(FromForm)]
pub(super) struct ImageUploadForm<'a> {
    pub image: TempFile<'a>
}

#[post("/", data = "<data>")]
pub(super) async fn upload_image(
    token: Token,
    image_service: &State<ImageService>,
    mut data: Form<ImageUploadForm<'_>>
) -> Result<(), AobaError> {
    let Token(token) = token;
    image_service.upload_image(&mut data.image, token).await?;
    Ok(())
}