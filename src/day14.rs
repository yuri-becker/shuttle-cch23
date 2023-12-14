use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, routes, Route};
use rocket_dyn_templates::{context, Template};

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    content: String,
}

#[post("/unsafe", data = "<body>")]
fn render_unsafe(body: Json<Request>) -> Template {
    Template::render("day14_unsafe", context! {content: &body.content})
}

#[post("/safe", data = "<body>")]
fn render_safe(body: Json<Request>) -> Template {
    Template::render("day14_safe", context! {content: &body.content})
}

pub struct Day14 {}

impl Day14 {
    pub fn routes() -> Vec<Route> {
        routes![render_unsafe, render_safe]
    }
}
