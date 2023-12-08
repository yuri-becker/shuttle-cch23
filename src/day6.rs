use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{post, routes, Route};

#[derive(Serialize)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}
#[post("/", data = "<text>")]
fn count_elfs(text: &'_ str) -> Json<ElfCount> {
    ElfCount {
        elf: text.matches("elf").count(),
        elf_on_a_shelf: text.matches("elf on a shelf").count(),
        shelf_with_no_elf_on_it: text.matches("shelf").count()
            - text.matches("elf on a shelf").count(),
    }
    .into()
}
pub struct Day6 {}

impl Day6 {
    pub fn routes() -> Vec<Route> {
        routes![count_elfs]
    }
}
