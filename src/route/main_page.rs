/*
 * @Author: Image image@by.cx
 * @Date: 2021-06-25 08:51:37
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2023-12-18 14:26:12
 * @filePathColon: /
 * @Description: 
 * 
 * Copyright (c) 2023 by Image, All Rights Reserved. 
 */
pub mod main_page{
    use warp::{Filter,reply};
    use rust_embed::RustEmbed;
    use warp::http::header;

    #[derive(RustEmbed)]
    #[folder = "www"]
    struct Asset;

    pub fn index_static()-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::path::end().map(||{
            let index = Asset::get("index.html").unwrap().data;
            reply::html(index)
        })
    }
    pub fn css_static()-> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::path("assets")
            .and(warp::path("css"))
            .and(warp::path("styles.css"))
            .map(||{
            let css = Asset::get("assets/css/styles.css").unwrap().data;
            reply::html(css)
        }).with(reply::with::header(header::CONTENT_TYPE, "text/css"))
    }
}