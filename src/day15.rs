use regex::Regex;
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{post, routes, Route};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum NiceResult {
    #[serde(rename = "nice")]
    Nice,
    #[serde(rename = "naughty")]
    Naughty,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    result: NiceResult,
}

#[derive(Serialize, Deserialize, Debug)]
struct GameResponse {
    result: NiceResult,
    reason: String,
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

#[post("/nice", data = "<request>")]
fn nice(request: Json<Request>) -> Result<Json<Response>, BadRequest<Json<Response>>> {
    println!("Got a nice request: {:?}", request);
    let contains_at_least_three_vowels = request
        .input
        .to_lowercase()
        .chars()
        .filter(|c| VOWELS.contains(c))
        .count()
        >= 3;
    let has_letters_that_appear_twice_in_a_row =
        Day15::has_letters_that_appear_twice_in_a_row(&request.input);
    let has_forbidden_substrings = FORBIDDEN_SUBSTRINGS
        .iter()
        .any(|s| request.input.contains(s));
    let result = if contains_at_least_three_vowels
        && has_letters_that_appear_twice_in_a_row
        && !has_forbidden_substrings
    {
        NiceResult::Nice
    } else {
        NiceResult::Naughty
    };

    match result {
        NiceResult::Nice => Ok(Json(Response { result })),
        NiceResult::Naughty => Err(BadRequest(Json(Response { result }))),
    }
}

#[post("/game", data = "<request>")]
fn game(request: Json<Request>) -> Result<Json<GameResponse>, (Status, Json<GameResponse>)> {
    if request.input.len() < 7 {
        return Err((
            Status::BadRequest,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "8 chars".to_string(),
            }),
        ));
    }

    if !request.input.chars().any(|c| c.is_uppercase())
        || !request.input.chars().any(|c| c.is_lowercase())
        || !request.input.chars().any(|c| c.is_ascii_digit())
    {
        return Err((
            Status::BadRequest,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "more types of chars".to_string(),
            }),
        ));
    }

    if request.input.chars().filter(|c| c.is_ascii_digit()).count() < 5 {
        return Err((
            Status::BadRequest,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "55555".to_string(),
            }),
        ));
    }

    if Day15::find_integers(&request.input).iter().sum::<i32>() != 2023 {
        return Err((
            Status::BadRequest,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "math is hard".to_string(),
            }),
        ));
    }

    if !Day15::is_joyful(&request.input) {
        return Err((
            Status::NotAcceptable,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "not joyful enough".to_string(),
            }),
        ));
    }

    if !Day15::contains_sandwich(&request.input) {
        return Err((
            Status::UnavailableForLegalReasons,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "illegal: no sandwich".to_string(),
            }),
        ));
    }

    if !request
        .input
        .chars()
        .any(|c| matches!(c, '\u{2980}'..='\u{2BFF}'))
    {
        return Err((
            Status::RangeNotSatisfiable,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "outranged".to_string(),
            }),
        ));
    }

    let has_emoji = Regex::new(r"\p{Emoji_Presentation}")
        .unwrap()
        .is_match(&request.input);
    if !has_emoji {
        return Err((
            Status::UpgradeRequired,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "😳".to_string(),
            }),
        ));
    }

    if sha256::digest(&request.input).chars().nth_back(0).unwrap() != 'a' {
        return Err((
            Status::ImATeapot,
            Json(GameResponse {
                result: NiceResult::Naughty,
                reason: "not a coffee brewer".to_string(),
            }),
        ));
    }

    Ok(Json(GameResponse {
        result: NiceResult::Nice,
        reason: "that's a nice password".to_string(),
    }))
}

pub struct Day15 {}

impl Day15 {
    pub fn routes() -> Vec<Route> {
        routes![nice, game]
    }

    fn has_letters_that_appear_twice_in_a_row(input: &str) -> bool {
        let chars = input.to_lowercase().chars().collect::<Vec<char>>();
        for i in 0..chars.len() - 1 {
            if i != 0 && chars[i] == chars[i - 1] && chars[i].is_alphabetic() {
                return true;
            }
        }
        false
    }

    fn find_integers(input: &str) -> Vec<i32> {
        let chars = input.chars().collect::<Vec<char>>();
        let mut integers: Vec<i32> = Vec::new();
        let mut last_digits = Vec::new();
        for i in 0..chars.len() {
            if chars[i].is_ascii_digit() {
                last_digits.push(chars[i]);
            }
            if !last_digits.is_empty()
                && (chars.get(i + 1).is_none() || !chars[i + 1].is_ascii_digit())
            {
                integers.push(
                    last_digits
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                );
                last_digits.clear();
            }
        }
        integers
    }

    fn is_joyful(input: &str) -> bool {
        let chars = input.to_lowercase().chars().collect::<Vec<char>>();

        let mut found = (false, false, false);
        for c in chars {
            if c == 'j' {
                if found.0 || found.1 || found.2 {
                    return false;
                }
                found.0 = true;
            }
            if c == 'o' {
                if !found.0 || found.1 || found.2 {
                    return false;
                }
                found.1 = true;
            }
            if c == 'y' {
                if !found.0 || !found.1 || found.2 {
                    return false;
                }
                found.2 = true;
            }
        }
        found.0 && found.1 && found.2
    }

    fn contains_sandwich(input: &str) -> bool {
        let chars = input.chars().collect::<Vec<char>>();
        for i in 0..chars.len() {
            if chars.get(i + 2).is_some()
                && chars[i].is_alphabetic()
                && chars[i + 2].is_alphabetic()
                && chars[i] == chars[i + 2]
                && chars[i + 1].is_alphabetic()
            {
                return true;
            }
        }
        false
    }
}
