use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

use rocket::futures::StreamExt;
use rocket::serde::json::serde_json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, routes, Route, Shutdown, State};
use rocket_ws::{Message, Stream, WebSocket};
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::{
    select,
    sync::broadcast::{channel, Sender},
};

#[derive(PartialEq, Debug)]
enum PingState {
    None,
    Started,
}

#[get("/ws/ping")]
fn ping(ws: WebSocket, day19: &State<Day19>) -> Stream!['_] {
    Stream! { ws => {
        for await message in ws {
            match message {
                Ok(message) => {
                    if message.is_close() {
                        break;
                    }
                    let response = day19.ping_pong(message);
                    if response.is_some() {
                        yield response.unwrap()
                    }
                },
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    }}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Twit {
    message: String,
}

#[derive(Debug, Clone)]
struct Broadcast {
    room: i32,
    author: String,
    twit: Twit,
}

impl From<(i32, String, Twit)> for Broadcast {
    fn from(value: (i32, String, Twit)) -> Self {
        Broadcast {
            room: value.0,
            author: value.1,
            twit: value.2,
        }
    }
}

#[post("/reset")]
fn bird_app_reset(day19: &State<Day19>) -> () {
    day19.reset();
}

#[get("/views")]
fn bird_app_views(day19: &State<Day19>) -> String {
    let string = day19.views().to_string();
    println!("views: {}", string);
    string
}

#[get("/ws/room/<room>/user/<user>")]
fn bird_app_connect<'a>(
    ws: WebSocket,
    day19: &'a State<Day19>,
    room: i32,
    user: &'a str,
    mut end: Shutdown,
) -> Stream!['a] {
    let mut rx = day19.stream.0.subscribe();
    Stream! { ws =>
        let mut ws = ws;
        loop {
            select! {
                res = ws.next() => {
                    match res {
                        Some(Ok(ref message)) => {
                            match message {
                                Message::Ping(ping) => yield Message::Pong(ping.clone()),
                                Message::Text(ref message) => {
                                    match serde_json::from_str::<Twit>(message) {
                                        Ok(twit) => {
                                            if twit.message.len() <= 128 {
                                                let  _ = day19.stream.0.send((room, user.to_string(), twit).into());
                                            }
                                        },
                                    _ => {}}
                                },
                                Message::Close(_) => break,
                                _ => {}
                            };
                        }
                    _ => break}
                },
                res = rx.recv() => {
                    match res {
                        Ok(ref twit) => {
                            if twit.room == room {
                                day19.log_view();
                                let response = serde_json::json!({
                                    "user": twit.author.clone(),
                                    "message": twit.twit.message.clone(),
                                });
                                let response = serde_json::to_string(&response).unwrap();
                                yield Message::Text(response);
                            }
                        },
                        Err(RecvError::Closed) => break,
                        Err(RecvError::Lagged(_)) => continue,
                    }
                },
                _ = &mut end => break

            }
       }
    }
}

pub struct Day19 {
    ping_state: RwLock<PingState>,
    twit_views: AtomicU64,
    stream: (Sender<Broadcast>, Receiver<Broadcast>),
}

impl Day19 {
    pub fn routes() -> Vec<Route> {
        routes![ping, bird_app_reset, bird_app_views, bird_app_connect]
    }

    fn ping_pong(&self, message: Message) -> Option<Message> {
        match message {
            Message::Text(text) => match text.as_str() {
                "serve" => {
                    let mut ping_state = self.ping_state.write().unwrap();
                    *ping_state = PingState::Started;
                    None
                }
                "ping" => {
                    let it = self.ping_state.read().unwrap();
                    if *it == PingState::Started {
                        Some(Message::Text(String::from("pong")))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn reset(&self) {
        self.twit_views.store(0, Ordering::Release);
    }

    fn views(&self) -> u64 {
        self.twit_views.load(Ordering::Acquire)
    }

    fn log_view(&self) {
        self.twit_views
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |views| Some(views + 1))
            .expect("Failed to increment views");
    }
}

impl Default for Day19 {
    fn default() -> Self {
        Day19 {
            ping_state: RwLock::new(PingState::None),
            twit_views: AtomicU64::new(0),
            stream: channel::<Broadcast>(4096),
        }
    }
}
