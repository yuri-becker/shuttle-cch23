use rocket::http::Status;
use rocket::{get, routes};
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

use crate::day1::Day1;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day4::Day4;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day8::Day8;
use crate::day_negative_1::DayNegative1;
use crate::infrastructure::Infrastructure;

mod day1;
mod day11;
mod day12;
mod day13;
mod day4;
mod day6;
mod day7;
mod day8;
mod day_negative_1;
mod infrastructure;

#[get("/")]
fn index() -> Status {
    Status::Ok
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_shared_db::Postgres] postgres: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    Ok(rocket::build()
        .manage(Infrastructure { postgres, persist })
        .mount("/-1", DayNegative1::routes())
        .mount("/1", Day1::routes())
        .mount("/4", Day4::routes())
        .mount("/6", Day6::routes())
        .mount("/7", Day7::routes())
        .mount("/8", Day8::routes())
        .mount("/11", Day11::routes())
        .mount("/12", Day12::routes())
        .mount("/13", Day13::routes())
        .mount("/", routes![index])
        .into())
}
