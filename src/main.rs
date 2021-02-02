use crate::route::api::api::api_sample;
use crate::entity::config::config::{Config};
use fast_log::init_split_log;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;

mod database;
mod entity;
mod route;
#[tokio::main]
async fn main() {
    tokio::fs::create_dir_all("logs/").await.unwrap();
    let log_res = init_split_log("logs/",
                                 1000,
                                 LogSize::MB(4),
                                 false,
                                 RollingType::All,
                                 log::Level::Info,
                                 None,
                                 true
    );
    Config::load().await;
    println!("Hello, world!");
    warp::serve(api_sample())
        .run(([0,0,0,0],3030))
        .await;
}
