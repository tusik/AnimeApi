/*
 * @Author: Image image@by.cx
 * @Date: 2023-12-15 11:34:10
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2023-12-16 15:16:33
 * @filePathColon: /
 * @Description: 
 * 
 * Copyright (c) 2023 by Image, All Rights Reserved. 
 */
pub mod config {
    use serde::Deserialize;
    use serde::Serialize;
    pub static mut CONFIG: Option<Config> = None;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Config {
        pub system: SystemConfig,
        pub host: HostConfig,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemConfig {
        pub mongo_uri: String,
        pub path: String,
        pub origin_img: String,
        pub preview_img: String,
        pub webp_path: String,
        pub dev:bool,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostConfig {
        pub domain: Vec<String>,
        pub redis: String,
    }
    impl Config {
        pub async fn load() {
            let config_file = tokio::fs::read_to_string("./config.toml").await;
            let c: Config;
            match config_file {
                Ok(content) => {
                    c = toml::from_str(content.as_str()).unwrap();
                }
                Err(_) => {
                    panic!("error load config")
                }
            }
            unsafe {
                CONFIG = Some(c);
            }
            
        }
    }
}
