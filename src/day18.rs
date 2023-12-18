use rocket::futures::future;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, routes, Route, State};
use sqlx::{query, Encode, Executor, FromRow, PgPool, QueryBuilder, Row};

use crate::infrastructure::Infrastructure;

const SCHEMA: &str = "DROP TABLE IF EXISTS regions;
DROP TABLE IF EXISTS orders;

CREATE TABLE regions (
  id INT PRIMARY KEY,
  name VARCHAR(50)
);

CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
);";

#[derive(Serialize, Deserialize, Debug, FromRow, Encode)]
struct Region {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Encode)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i64,
}

#[post("/reset")]
async fn reset(infrastructure: &State<Infrastructure>) -> Status {
    infrastructure
        .postgres
        .execute(SCHEMA)
        .await
        .map(|_| Status::Ok)
        .unwrap_or_else(|err| {
            eprintln!("Could not reset database: {}", err);
            Status::InternalServerError
        })
}

#[post("/orders", data = "<orders>")]
async fn add_orders(orders: Json<Vec<Order>>, infrastructure: &State<Infrastructure>) -> Status {
    let orders: Vec<Order> = orders.0;
    if orders.is_empty() {
        return Status::Ok;
    }
    let mut query = QueryBuilder::new("INSERT INTO orders (id, region_id, gift_name, quantity) ");
    query.push_values(orders, |mut b, order| {
        b.push_bind(order.id)
            .push_bind(order.region_id)
            .push_bind(order.gift_name)
            .push_bind(order.quantity);
    });
    query
        .build()
        .execute(&infrastructure.postgres)
        .await
        .map(|_| Status::Ok)
        .unwrap_or_else(|err| {
            eprintln!("Could not execute query: {}", err);
            Status::InternalServerError
        })
}

#[post("/regions", data = "<regions>")]
async fn add_regions(regions: Json<Vec<Region>>, infrastructure: &State<Infrastructure>) -> Status {
    if regions.is_empty() {
        return Status::Ok;
    }
    let mut query = QueryBuilder::new("INSERT INTO regions (id, name) ");
    query.push_values(regions.0, |mut b, region| {
        b.push_bind(region.id).push_bind(region.name);
    });
    query
        .build()
        .execute(&infrastructure.postgres)
        .await
        .map(|_| Status::Ok)
        .unwrap_or_else(|err| {
            eprintln!("Could not execute query: {}", err);
            Status::InternalServerError
        })
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
struct TotalOrdersPerRegion {
    region: String,
    total: i64,
}

#[get("/regions/total")]
async fn total_regions(
    infrastructure: &State<Infrastructure>,
) -> Result<Json<Vec<TotalOrdersPerRegion>>, Status> {
    query(
        "SELECT name as region, SUM(quantity) as total 
FROM orders 
JOIN regions ON orders.region_id=regions.id
WHERE quantity != 0
GROUP BY (regions.id)
ORDER BY regions.name ASC;",
    )
    .fetch_all(&infrastructure.postgres)
    .await
    .map(|it| {
        it.iter()
            .map(TotalOrdersPerRegion::from_row)
            .map(Result::unwrap)
            .collect::<Vec<TotalOrdersPerRegion>>()
    })
    .map(Json::from)
    .map_err(|err| {
        eprintln!("Could not execute query: {}", err);
        Status::InternalServerError
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct TopGifts {
    region: String,
    top_gifts: Vec<String>,
}

#[get("/regions/top_list/<top>")]
async fn top_per_region(
    top: i32,
    infrastructure: &State<Infrastructure>,
) -> Result<Json<Vec<TopGifts>>, Status> {
    let regions = query("SELECT * from regions ORDER BY name ASC;")
        .fetch_all(&infrastructure.postgres)
        .await
        .map(|it| {
            it.iter()
                .map(Region::from_row)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .map_err(|err| {
            eprintln!("Could not execute query: {}", err);
            Status::InternalServerError
        })?;

    let top_gifts = future::join_all(
        regions
            .iter()
            .map(|region| Day18::top_gifts_for_region(region, &top, &infrastructure.postgres)),
    )
    .await;
    Ok(Json(top_gifts))
}

pub struct Day18 {}

impl Day18 {
    pub fn routes() -> Vec<Route> {
        routes![
            reset,
            add_orders,
            total_regions,
            add_regions,
            top_per_region
        ]
    }

    async fn top_gifts_for_region(region: &Region, top: &i32, executor: &PgPool) -> TopGifts {
        let gift_names = query("SELECT gift_name FROM orders WHERE region_id=$1 GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT $2;")
            .bind(region.id).bind(top)
            .fetch_all(executor)
            .await
            .map(|rows| {
                rows.iter()
                    .map(|row| row.get::<String, &'static str>("gift_name"))
                    .collect::<Vec<String>>()
            })
            .unwrap();
        TopGifts {
            region: String::from(&region.name),
            top_gifts: gift_names,
        }
    }
}
