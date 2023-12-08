use rocket::{get, Route, routes};
use rocket::http::Status;
use rocket::http::uri::fmt::Path;
use rocket::http::uri::Segments;

#[get("/<num..>")]
fn sled_id<'r>(num: Segments<'r, Path>) -> Result<String, Status> {
    if num.len() > 20 {
        return Err(Status::BadRequest);
    }
    let nums = match num.into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>() {
        Ok(nums) => nums,
        Err(_) => return Err(Status::BadRequest)
    };

    match nums.iter().map(|&it| it).reduce(|a, b| a ^ b).map(|it| it.pow(3)) {
        Some(it) => Ok(it.to_string()),
        None => Err(Status::BadRequest)
    }
}


pub struct Day1 {}

impl Day1 {
    pub fn routes() -> Vec<Route> {
        routes![sled_id]
    }
}