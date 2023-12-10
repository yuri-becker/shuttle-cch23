use base64::{engine::general_purpose, Engine};
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::serde::{json, Serialize};
use rocket::{get, routes, Route};
use serde::Deserialize;
use std::collections::HashMap;

#[get("/decode")]
pub fn decode(cookies: &CookieJar<'_>) -> Result<String, Status> {
    match cookies.get("recipe") {
        None => Err(Status::BadRequest),
        Some(recipe) => general_purpose::STANDARD
            .decode(recipe.value())
            .map(|it| String::from_utf8(it).unwrap())
            .map_err(|_| Status::BadRequest),
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Ingredients {
    #[serde(flatten)]
    extra: HashMap<String, i32>,
}
#[derive(Deserialize, Serialize, Debug)]
struct Request {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Deserialize, Serialize)]
struct Response {
    cookies: i32,
    pantry: Ingredients,
}

#[get("/bake")]
fn bake(cookies: &CookieJar<'_>) -> Result<Json<Response>, Status> {
    let request = match cookies.get("recipe") {
        None => return Err(Status::BadRequest),
        Some(recipe) => general_purpose::STANDARD
            .decode(recipe.value())
            .map(|it| String::from_utf8(it).unwrap())
            .map_err(|_| Status::BadRequest),
    };

    let request = match request {
        Err(_) => return Err(Status::BadRequest),
        Ok(recipe) => recipe,
    };

    let request: Request = match json::from_str(&request) {
        Err(_) => return Err(Status::BadRequest),
        Ok(recipe) => recipe,
    };
    println!("request: {:?}", request);

    let makeable_cookies_per_ingredient = request
        .recipe
        .extra
        .iter()
        .map(|(key, value)| {
            if value.clone() == 0 {
                (key, 0)
            } else {
                (key, request.pantry.extra.get(key).unwrap_or(&0) / value)
            }
        })
        .collect::<HashMap<_, _>>();

    let ingredient_limit = makeable_cookies_per_ingredient.values().min().unwrap_or(&0);

    Ok(Json::from(Response {
        cookies: *ingredient_limit,
        pantry: Ingredients {
            extra: request
                .pantry
                .extra
                .iter()
                .map(|(key, value)| {
                    (
                        String::from(key),
                        value - ingredient_limit * request.recipe.extra.get(key).unwrap_or(&0),
                    )
                })
                .collect::<HashMap<_, _>>(),
        },
    }))
}

pub struct Day7 {}

impl Day7 {
    pub fn routes() -> Vec<Route> {
        routes![decode, bake]
    }
}
