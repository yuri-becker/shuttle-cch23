use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::{post, routes};
use rocket::{response, Request, Route};

enum SliceResult<'r> {
    Unsplit(Json<Vec<&'r str>>),
    Split(Json<Vec<Vec<&'r str>>>),
}

impl<'r> Responder<'r, 'static> for SliceResult<'r> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            SliceResult::Unsplit(json) => json.respond_to(request),
            SliceResult::Split(json) => json.respond_to(request),
        }
    }
}

#[post("/?<offset>&<limit>&<split>", data = "<data>")]
fn slice(
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
    data: Json<Vec<&str>>,
) -> SliceResult {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(data.0.len());
    let take = data
        .0
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();
    if split.is_none() {
        return SliceResult::Unsplit(Json(take));
    }
    let split = split.unwrap();
    SliceResult::Split(Json(
        take.chunks(split)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>(),
    ))
}
pub struct Day5 {}

impl Day5 {
    pub fn routes() -> Vec<Route> {
        routes![slice]
    }
}
