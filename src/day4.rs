use rocket::serde::{json::Json, Deserialize};

use rocket::http::Status;
use rocket::{post, routes, Route};
use serde::Serialize;

#[derive(Deserialize)]
struct Reindeer<'r> {
    name: &'r str,
    strength: i32,
}

#[derive(Deserialize)]
struct ContestParticipant<'r> {
    #[serde(flatten)]
    reindeer: Reindeer<'r>,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: &'r str,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

#[post("/strength", data = "<reindeers>")]
fn strength(reindeers: Json<Vec<Reindeer<'_>>>) -> String {
    reindeers
        .iter()
        .map(|reindeer| reindeer.strength)
        .sum::<i32>()
        .to_string()
}

#[post("/contest", data = "<contest_participants>")]
fn contest<'r>(
    contest_participants: Json<Vec<ContestParticipant<'r>>>,
) -> Result<Json<ContestResults>, Status> {
    let (fastest, tallest, magician, consumer) = (
        contest_participants
            .iter()
            .max_by(|a, b| a.speed.total_cmp(&b.speed)),
        contest_participants.iter().max_by_key(|&a| a.height),
        contest_participants
            .iter()
            .max_by_key(|a| a.snow_magic_power),
        contest_participants
            .iter()
            .max_by_key(|a| a.candies_eaten_yesterday),
    );

    if [fastest, tallest, magician, consumer]
        .iter()
        .any(|x| x.is_none())
    {
        return Err(Status::BadRequest);
    }
    let (fastest, tallest, magician, consumer) = (
        fastest.unwrap(),
        tallest.unwrap(),
        magician.unwrap(),
        consumer.unwrap(),
    );
    Ok(ContestResults {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.reindeer.strength, fastest.reindeer.name,
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.reindeer.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.reindeer.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.reindeer.name, consumer.favorite_food,
        ),
    }
    .into())
}
pub struct Day4 {}

impl Day4 {
    pub fn routes() -> Vec<Route> {
        routes![strength, contest]
    }
}
