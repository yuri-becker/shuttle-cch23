use std::time::SystemTime;

use chrono::{DateTime, Datelike, Utc, Weekday};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{get, post, routes, Route, State};
use serde::Serialize;
use shuttle_persist::PersistInstance;
use ulid::Ulid;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct UlidGenerationDatesResponse {
    #[serde(rename = "christmas eve")]
    christmas_eve: usize,
    weekday: usize,
    #[serde(rename = "in the future")]
    in_the_future: usize,
    #[serde(rename = "LSB is 1")]
    lsb_is_one: usize,
}

#[post("/save/<key>")]
fn store(key: &str, persist: &State<PersistInstance>) -> Status {
    persist
        .save(key, SystemTime::now())
        .map(|_| Status::Ok)
        .unwrap_or(Status::BadRequest)
}

#[get("/load/<key>")]
fn load(key: &str, persist: &State<PersistInstance>) -> Result<String, Status> {
    let persisted_value = persist.load::<SystemTime>(key);
    println!("Persisted value: {:?}", persisted_value);
    persisted_value
        .map(|value| SystemTime::now().duration_since(value).unwrap())
        .map(|duration| duration.as_secs().to_string())
        .map_err(|_| Status::NotFound)
}

#[post("/ulids", data = "<ulids>")]
fn ulids(ulids: Json<Vec<String>>) -> Result<Json<Vec<String>>, Status> {
    let parsed_ulids = Day12::parse_ulids(ulids)?;
    let mut uuids = parsed_ulids
        .iter()
        .map(|&ulid| {
            let bytes = ulid.into();
            Uuid::from_bytes(bytes).to_string()
        })
        .collect::<Vec<String>>();
    uuids.reverse();
    Ok(Json::from(uuids))
}

#[post("/ulids/<weekday>", data = "<ulids>")]
fn generation_dates(
    weekday: u8,
    ulids: Json<Vec<String>>,
) -> Result<Json<UlidGenerationDatesResponse>, Status> {
    let parsed_ulids = Day12::parse_ulids(ulids)?;
    Ok(Json::from(UlidGenerationDatesResponse {
        christmas_eve: parsed_ulids
            .iter()
            .filter(|ulid| Day12::is_christmas_eve(&ulid.datetime()))
            .count(),
        weekday: parsed_ulids
            .iter()
            .filter(|ulid| Day12::is_weekday(&ulid.datetime(), weekday))
            .count(),
        in_the_future: parsed_ulids
            .iter()
            .filter(|ulid| Day12::is_in_the_future(&ulid.datetime()))
            .count(),
        lsb_is_one: parsed_ulids
            .iter()
            .filter(|ulid| ulid.random() & 1 == 1)
            .count(),
    }))
}

pub struct Day12 {}

impl Day12 {
    pub fn routes() -> Vec<Route> {
        routes![store, load, ulids, generation_dates]
    }

    fn parse_ulids(ulids: Json<Vec<String>>) -> Result<Vec<Ulid>, Status> {
        let parsed_ulids: Result<Vec<Ulid>, _> =
            ulids.iter().map(|ulid| Ulid::from_string(ulid)).collect();
        if parsed_ulids.is_err() {
            return Err(Status::BadRequest);
        }
        Ok(parsed_ulids.unwrap())
    }

    fn is_christmas_eve(datetime: &SystemTime) -> bool {
        let datetime: DateTime<Utc> = datetime.to_owned().into();
        return datetime.month() == 12 && datetime.day() == 24;
    }

    fn is_weekday(datetime: &SystemTime, weekday: u8) -> bool {
        let datetime: DateTime<Utc> = datetime.to_owned().into();
        match datetime.weekday() {
            Weekday::Mon => weekday == 0,
            Weekday::Tue => weekday == 1,
            Weekday::Wed => weekday == 2,
            Weekday::Thu => weekday == 3,
            Weekday::Fri => weekday == 4,
            Weekday::Sat => weekday == 5,
            Weekday::Sun => weekday == 6,
        }
    }

    fn is_in_the_future(datetime: &SystemTime) -> bool {
        return datetime > &SystemTime::now();
    }
}
