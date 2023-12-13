use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, routes, Route, State};
use sqlx::{query, Encode, Executor, FromRow, QueryBuilder, Row};

use crate::infrastructure::Infrastructure;

const SCHEMA: &str = "DROP TABLE IF EXISTS orders;
CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
);";

#[get("/sql")]
async fn sql(infrastructure: &State<Infrastructure>) -> Result<String, Status> {
    let value: i32 = query("SELECT 20231213;")
        .fetch_one(&infrastructure.postgres)
        .await
        .map_err(|err| {
            eprintln!("Could not execute query: {}", err);
            Status::InternalServerError
        })?
        .get(0);
    Ok(value.to_string())
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

#[derive(Serialize, Deserialize, Debug)]
struct TotalOrders {
    total: i64,
}
#[get("/orders/total")]
async fn total_orders(infrastructure: &State<Infrastructure>) -> Result<Json<TotalOrders>, Status> {
    // Create a query that adds up all order.quantity values
    let total: i64 = query("SELECT SUM(quantity) FROM orders;")
        .fetch_one(&infrastructure.postgres)
        .await
        .map_err(|err| {
            eprintln!("Could not execute query: {}", err);
            Status::InternalServerError
        })?
        .get(0);

    Ok(TotalOrders { total }.into())
}

#[derive(Serialize, Deserialize, Debug)]
struct MostPopular {
    popular: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
struct QueryResult {
    gift_name: String,
    total_quantity: i64,
}

#[get("/orders/popular")]
async fn most_popular(infrastructure: &State<Infrastructure>) -> Result<Json<MostPopular>, Status> {
    let result: Vec<QueryResult> = query(
        "SELECT gift_name, SUM(quantity) as total_quantity FROM orders GROUP BY gift_name ORDER BY total_quantity DESC LIMIT 1;",
    )
    .fetch_all(&infrastructure.postgres)
    .await
    .map(|row| {
        row.iter()
            .map(QueryResult::from_row)
            .map(Result::unwrap) // If this panics, the query does not match the struct
            .collect::<Vec<QueryResult>>()
    })
    .map_err(|err| {
        eprintln!("Could not execute query: {}", err);
        Status::InternalServerError
    })?;
    if result.is_empty() {
        return Ok(MostPopular { popular: None }.into());
    }
    Ok(MostPopular {
        popular: Some(result[0].gift_name.clone()),
    }
    .into())
}

pub struct Day13 {}

impl Day13 {
    pub fn routes() -> Vec<Route> {
        routes![reset, sql, add_orders, total_orders, most_popular]
    }
}
