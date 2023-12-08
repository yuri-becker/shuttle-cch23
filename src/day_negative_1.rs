use rocket::{get, Route, routes};
use rocket::http::Status;

#[get("/error")]
fn fake_error() -> Status {
    Status::InternalServerError
}

pub struct DayNegative1 {}

impl DayNegative1 {
    pub fn routes() -> Vec<Route> {
        routes![fake_error]
    }
}