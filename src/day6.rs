use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{post, routes, Route};

#[derive(Serialize, Debug)]
struct ElfCount {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}
#[post("/", data = "<text>")]
fn count_elfs(text: &'_ str) -> Json<ElfCount> {
    let elf_on_a_shelf =
        text.matches("elf on a shelf").count() + text.matches("shelf on a shelf").count();
    ElfCount {
        elf: text.matches("elf").count(),
        elf_on_a_shelf,
        shelf_with_no_elf_on_it: text.matches("shelf").count() - elf_on_a_shelf,
    }
    .into()
}
pub struct Day6 {}

impl Day6 {
    pub fn routes() -> Vec<Route> {
        routes![count_elfs]
    }
}
