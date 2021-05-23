use crate::route::api::api::{api_sample, api_sample_post};
use crate::entity::config::config::{Config};
use fast_log::init_split_log;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use warp::{Filter};
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
    match log_res{
        Ok(_) => {}
        Err(_) => {}
    }
    Config::load().await;
    println!("Hello, world!");
    let routes = api_sample_post().or(api_sample());
    warp::serve(routes)
        .run(([0,0,0,0],3030))
        .await;
}
