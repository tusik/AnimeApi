#![allow(non_snake_case)]
use crate::entity::config::config::Config;
use crate::route::api::api::{
    api_image_cors, api_imagejson_cors, api_images_cors, api_sample, api_sample_json,
    api_sample_post, api_sample_red_post, api_server_status, api_tags,
};
use crate::route::main_page::main_page::{css_static, index_static};
use fast_log::plugin::file_split::{DateType, KeepType, Rolling, RollingType};
use fast_log::plugin::packer::LogPacker;
use warp::Filter;
use crate::database::handler::handler::{init_mongo, init_query_cache, init_redis};
use std::env;

mod database;
mod entity;
mod route;
mod util;
#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "warp=info");
    tokio::fs::create_dir_all("logs/").await.unwrap();

    fast_log::init(fast_log::config::Config::new().console().file_split(
        "logs/",
        Rolling::new(RollingType::ByDate(DateType::Day)),
        KeepType::KeepNum(5),
        LogPacker {},
    ))
    .unwrap();

    Config::load().await;
    init_redis();
    init_query_cache();
    init_mongo().await;
    println!("Hello, world!");
    let image_static = warp::path!("static" / "image" / ..).and(warp::fs::dir("./www"));
    let routes = api_sample_post()
        .or(api_sample_red_post())
        .or(api_sample_json())
        .or(api_sample())
        .or(api_server_status())
        .or(api_image_cors())
        .or(api_imagejson_cors())
        .or(api_images_cors())
        .or(api_tags())
        .or(index_static())
        .or(css_static())
        .or(image_static);
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
