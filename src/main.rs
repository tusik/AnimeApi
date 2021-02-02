use crate::route::api::api::api_sample;
use crate::entity::config::config::{SystemConfig, Config};

mod database;
mod entity;
mod route;
#[tokio::main]
async fn main() {
    Config::load().await;
    println!("Hello, world!");
    warp::serve(api_sample())
        .run(([0,0,0,0],3030))
        .await;
}
