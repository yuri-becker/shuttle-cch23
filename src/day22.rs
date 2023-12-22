use pathfinding::directed::bfs::bfs;
use rocket::http::Status;
use rocket::{post, routes, Route};

#[post("/integers", data = "<text>")]
fn integers(text: String) -> Result<String, Status> {
    Day22::integers(text)
}

#[post("/rocket", data = "<text>")]
fn rocket_path(text: String) -> Result<String, Status> {
    Day22::rocket(text)
}

pub struct Day22 {}

impl Day22 {
    pub fn routes() -> Vec<Route> {
        routes![integers, rocket_path]
    }

    fn integers(text: String) -> Result<String, Status> {
        let lines = text
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Status::BadRequest)?;

        let result = lines.iter().fold(0u64, |acc, &x| acc ^ x);
        Ok("🎁".repeat(result as usize))
    }
    fn rocket(chart: String) -> Result<String, Status> {
        let chart = Day22::parse_chart(chart)?;
        let graph = Day22::build_graph(&chart);
        let result = bfs(
            &0,
            |&node| graph[node].clone(),
            |node| *node == graph.len() - 1,
        )
        .expect("No path found");
        let mut distance = 0.0;

        for i in 0..result.len() - 1 {
            distance += chart.0[result[i]].distance(&chart.0[result[i + 1]]);
        }

        let distance = (distance * 1000.0).round() / 1000.0;

        Ok(format!("{} {:.3}", result.len() - 1, distance))
    }

    fn parse_chart(chart: String) -> Result<(Vec<Coordinate>, Vec<Portal>), Status> {
        let lines: Vec<&str> = chart.split('\n').collect();
        let stars_amount = lines[0]
            .parse::<usize>()
            .map_err(|_| {
                println!("Could not parse starts amount");
                Status::BadRequest
            })
            .and_then(|num| {
                if (2..=100).contains(&num) {
                    Ok(num)
                } else {
                    println!("Stars amount is not in range 2..=100");
                    Err(Status::BadRequest)
                }
            })?;
        let stars = lines
            .iter()
            .skip(1)
            .take(stars_amount)
            .map(|coordinate_line| {
                let numbers = coordinate_line.split(' ').collect::<Vec<&str>>();
                if numbers.len() != 3 {
                    return Err(Status::BadRequest);
                }
                numbers
                    .iter()
                    .map(|num| num.parse::<i32>())
                    .collect::<Result<Coordinate, _>>()
                    .map_err(|_| Status::BadRequest)
            })
            .collect::<Result<Vec<Coordinate>, _>>()?;

        let portals_amount = lines[stars_amount + 1]
            .parse::<usize>()
            .map_err(|_| {
                println!("Could not parse portals amount");
                Status::BadRequest
            })
            .and_then(|num| {
                if (1..=100).contains(&num) {
                    Ok(num)
                } else {
                    println!("Portals amount is not in range 1..=100");
                    Err(Status::BadRequest)
                }
            })?;

        let portals = lines
            .iter()
            .skip(2 + stars_amount)
            .take(portals_amount)
            .map(|portal_line| {
                let numbers = portal_line.split(' ').collect::<Vec<&str>>();
                if numbers.len() != 2 {
                    println!("Portal line does not have 2 numbers");
                    return Err(Status::BadRequest);
                }
                numbers
                    .iter()
                    .map(|num| num.parse::<usize>())
                    .collect::<Result<Portal, _>>()
                    .map_err(|_| {
                        println!("Could not parse portal");
                        Status::BadRequest
                    })
            })
            .collect::<Result<Vec<Portal>, _>>()?;

        Ok((stars, portals))
    }

    fn build_graph(chart: &(Vec<Coordinate>, Vec<Portal>)) -> Graph {
        println!("Chart Length: {}", chart.0.len());
        let mut graph = Graph::with_capacity(chart.0.len());
        for i in 0..chart.0.len() {
            let left_connections = chart
                .1
                .iter()
                .filter(|portal| portal.a == i)
                .map(|portal| portal.b)
                .collect::<Vec<CoordinateIndex>>();
            let right_connections = chart
                .1
                .iter()
                .filter(|portal| portal.b == i)
                .map(|portal| portal.a)
                .collect::<Vec<CoordinateIndex>>();
            // concat left and right connections
            graph.push(
                left_connections
                    .iter()
                    .chain(right_connections.iter())
                    .cloned()
                    .collect::<Vec<CoordinateIndex>>(),
            );
        }
        println!("Graph: {:?}", graph);
        graph
    }
}

#[derive(Debug)]
struct Portal {
    a: usize,
    b: usize,
}

impl FromIterator<usize> for Portal {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        const ERROR_MESSAGE: &str = "Please make sure that the iterator has 2 elements";
        let it = Vec::from_iter(iter);
        Portal {
            a: *it.first().expect(ERROR_MESSAGE),
            b: *it.get(1).expect(ERROR_MESSAGE),
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> f32 {
        let y = ((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2))
            as f32;
        y.sqrt()
    }
}

impl FromIterator<i32> for Coordinate {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        const ERROR_MESSAGE: &str = "Please make sure that the iterator has 3 elements";
        let iter = Vec::from_iter(iter);

        Coordinate {
            x: *iter.first().expect(ERROR_MESSAGE),
            y: *iter.get(1).expect(ERROR_MESSAGE),
            z: *iter.get(2).expect(ERROR_MESSAGE),
        }
    }
}

type CoordinateIndex = usize;
type Graph = Vec<Vec<CoordinateIndex>>;
