use rocket::data::{ByteUnit, Limits};
use rocket::http::Status;
use rocket::{get, routes, Config};
use rocket_dyn_templates::Template;
use shuttle_persist::PersistInstance;
use sqlx::PgPool;

use crate::day1::Day1;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day14::Day14;
use crate::day15::Day15;
use crate::day18::Day18;
use crate::day19::Day19;
use crate::day20::Day20;
use crate::day21::Day21;
use crate::day22::Day22;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day8::Day8;
use crate::day_negative_1::DayNegative1;
use crate::infrastructure::Infrastructure;

mod day1;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day4;
mod day5;
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
        .manage(Day19::default())
        .manage(Day21::default())
        .mount("/-1", DayNegative1::routes())
        .mount("/1", Day1::routes())
        .mount("/4", Day4::routes())
        .mount("/5", Day5::routes())
        .mount("/6", Day6::routes())
        .mount("/7", Day7::routes())
        .mount("/8", Day8::routes())
        .mount("/11", Day11::routes())
        .mount("/12", Day12::routes())
        .mount("/13", Day13::routes())
        .mount("/14", Day14::routes())
        .mount("/15", Day15::routes())
        .mount("/18", Day18::routes())
        .mount("/19", Day19::routes())
        .mount("/20", Day20::routes())
        .mount("/21", Day21::routes())
        .mount("/22", Day22::routes())
        .mount("/", routes![index])
        .attach(Template::fairing())
        .configure(Config {
            limits: Limits::default()
                .limit("file", ByteUnit::Megabyte(512))
                .limit("string", ByteUnit::Megabyte(512)),
            ..Default::default()
        })
        .into())
}
