use rocket::http::Status;
use rocket::{get, routes};

use crate::day1::Day1;
use crate::day4::Day4;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day_negative_1::DayNegative1;

mod day1;
mod day4;
mod day6;
mod day7;
mod day_negative_1;

#[get("/")]
fn index() -> Status {
    Status::Ok
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    Ok(rocket::build()
        .mount("/-1", DayNegative1::routes())
        .mount("/1", Day1::routes())
        .mount("/4", Day4::routes())
        .mount("/6", Day6::routes())
        .mount("/7", Day7::routes())
        .mount("/", routes![index])
        .into())
}
