use std::io::Cursor;

use image::GenericImageView;
use rocket::form::Form;
use rocket::fs::{relative, NamedFile, TempFile};
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;
use rocket::{get, post, routes, FromForm, Route};

#[get("/assets/decoration.png")]
async fn decoration() -> Result<NamedFile, Status> {
    let file = relative!("assets/decoration.png");
    NamedFile::open(file).await.map_err(|e| {
        eprintln!("Could not open decoration.png: {:?}", e);
        Status::InternalServerError
    })
}

#[derive(FromForm)]
struct RedPixelsRequest<'r> {
    image: TempFile<'r>,
}

#[post("/red_pixels", data = "<request>")]
async fn count_red_pixels(request: Form<RedPixelsRequest<'_>>) -> Result<String, Status> {
    let mut buffer = Vec::new();
    request
        .image
        .open()
        .await
        .map_err(|e| {
            eprintln!("Could not open image: {:?}", e);
            Status::InternalServerError
        })?
        .read_to_end(&mut buffer)
        .await
        .map_err(|e| {
            eprintln!("Could not read image: {:?}", e);
            Status::InternalServerError
        })?;

    let image = image::io::Reader::new(Cursor::new(buffer))
        .with_guessed_format()
        .map_err(|e| {
            eprintln!("Could not guess format: {:?}", e);
            Status::InternalServerError
        })?
        .decode()
        .map_err(|e| {
            eprintln!("Could not decode image: {:?}", e);
            Status::BadRequest
        })?;
    let red_pixels = image
        .pixels()
        .filter(|(_x, _y, pixel)| {
            let red = pixel[0] as u16;
            let green = pixel[1] as u16;
            let blue = pixel[2] as u16;
            red > green + blue
        })
        .count();
    Ok(red_pixels.to_string())
}

pub struct Day11 {}

impl Day11 {
    pub fn routes() -> Vec<Route> {
        routes![decoration, count_red_pixels]
    }
}
