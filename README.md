[![Unlicense license](https://img.shields.io/github/license/yuri-becker/shuttle-cch23?style=for-the-badge&logo=unlicense&logoColor=white
)](https://github.com/yuri-becker/shuttle-cch23/blob/main/LICENSE)

<br />
<div align="center">

  <h1 align="center"><strong>shuttle-cch23</strong></h1>

  <p align="center">
    My solutions for the <a href="https://www.shuttle.rs/cch"> Shuttle Christmas Code Hunt 2023</a>.<br/>
    May include messy code, but feel free to use code in this repo.
  </p>
</div>

## Content

Some code may serve as examples for how to do something specific in Rust. For future reference, I summarized the challenges. Every challenge uses [rocket](https://crates.io/crates/rocket) and [serde](https://crates.io/crates/serde).

| File | Topics | Crates |
| ---- | ----------- | ------ |
| [day_negative_1.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day_negative_1.rs) | Error status codes | | 
| [day1.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day1.rs) | Dynamic path parameters | |
| [day4.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day4.rs) | JSON Serialisation | |
| [day5.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day5.rs) | Splitting and offsetting strings | |
| [day6.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day6.rs) | Counting substrings | |
| [day7.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day7.rs) | Cookies, HashMaps, base64 | [base64](https://crates.io/crates/base64) |
| [day8.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day8.rs) | Http requests | [reqwest](https://crates.io/crates/reqwest) |
| [day11.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day11.rs) | Serving static files, accepting files, parsing images  | [image](https://crates.io/crates/image) 
| [day12.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day12.rs) | ULIDs, UUIDs, DateTime operations | [chrono](https://crates.io/crates/chrono), [ulid](https://crates.io/crates/ulid), [uuid](https://crates.io/crates/uuid) |
| [day13.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day13.rs) | SQL Queries, SQL Groups, SQL Batch inserts | [sqlx](https://crates.io/crates/sqlx) |
| [day14.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day14.rs) | HTML Templates | [rocket_dyn_templates](https://crates.io/crates/rocket_dyn_templates) |
| [day15.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day15.rs) | Regex, Match Emojis, string matching | [regex](https://crates.io/crates/regex) |
| [day18.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day18.rs) | SQL Batch inserts, SQL joins, SQL Groups | [sqlx](https://crates.io/crates/sqlx) |
| [day19.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day19.rs) | Websocket, Broadcast, Atomic, RwLock | [rocket_ws](https://crates.io/crates/rocket_ws), [tokio](https://crates.io/crates/tokio) |
| [day20.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day20.rs) | Unpacking tars, finding git commits, accepting files, size limits, finding git file changes | [git2](https://crates.io/crates/git2), [tempfile](https://crates.io/crates/tempfile), [tar](https://crates.io/crates/tar) |
| [day21.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day21.rs) | [s2 cells](http://s2geometry.io), coordinate conversion, getting a coordinate's country | [s2](https://crates.io/crates/s2), [isocountry](https://crates.io/crates/isocountry), [country_boundaries](https://crates.io/crates/country-boundaries) |
| [day22.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/day22.rs) | Path-finding (breadth-first-search/bfs), Graphs | [pathfinding](https://crates.io/crates/pathfinding) |
| [tiebreaker.rs](https://github.com/yuri-becker/shuttle-cch23/blob/main/src/tiebreaker.rs) | JSON Web Encryption (JWE) | [josekit](https://crates.io/crates/josekit), [reqwest](https://crates.io/crates/reqwest) |

## Usage
```sh
 git clone git@github.com:yuri-becker/shuttle-cch23.git
 cd shuttle-cch23
 cargo install cargo-shuttle cargo-watch
 cargo watch -x "shuttle run"
 ```
