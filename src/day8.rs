use reqwest::StatusCode;
use rocket::http::Status;
use rocket::serde::Serialize;
use rocket::{get, routes, Route};
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug)]
struct PokeApiResponse {
    #[serde(rename = "weight")]
    weight_in_hectograms: i32,
}

const EARTH_GRAVITY: f64 = 9.825;

async fn get_pokedex_entry(pokedex_number: i32) -> Result<PokeApiResponse, Status> {
    let response = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{}/",
        pokedex_number
    ))
    .await;
    if response.is_err() {
        let err = response.unwrap_err();
        return match err.status() {
            Some(StatusCode::NOT_FOUND) => Err(Status::NotFound),
            _ => {
                eprint!("Request failed: {}", err);
                Err(Status::InternalServerError)
            }
        };
    };
    Ok(response.unwrap().json::<PokeApiResponse>().await.unwrap())
}

#[get("/weight/<pokedex_number>")]
pub async fn get_weight(pokedex_number: i32) -> Result<String, Status> {
    Ok((get_pokedex_entry(pokedex_number)
        .await?
        .weight_in_hectograms
        / 10)
        .to_string())
}

#[get("/drop/<pokedex_number>")]
pub async fn get_drop_momentum(pokedex_number: i32) -> Result<String, Status> {
    let weight_in_kg = get_pokedex_entry(pokedex_number)
        .await?
        .weight_in_hectograms as f64
        / 10.0;
    let velocity = f64::sqrt(2.0 * EARTH_GRAVITY * 10.0);
    let momentum = weight_in_kg * velocity;
    Ok(momentum.to_string())
}

pub struct Day8 {}

impl Day8 {
    pub fn routes() -> Vec<Route> {
        routes![get_weight, get_drop_momentum]
    }
}
