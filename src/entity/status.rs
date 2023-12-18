/*
 * @Author: Image_woker_pc image@by.cx
 * @Date: 2022-08-08 15:37:18
 * @LastEditors: Image image@by.cx
 * @LastEditTime: 2023-12-18 11:14:42
 * @FilePath: \AnimeApi\src\entity\status.rs
 * @filePathColon: /
 * @Description: 
 * 
 * Copyright (c) 2022 by Image_woker_pc image@by.cx, All Rights Reserved. 
 */
pub mod status{
    use serde::{Serialize, Deserialize};
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    #[derive(Debug,Serialize,Deserialize)]
    pub struct ImageBucket{
        pub count : u64,
        pub last_update: String,
        pub api_call_count:u64,
        pub traffic:u64
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct ServerStatus{
        pub status: u64,
        pub version: String,
        pub data:ImageBucket
        
    }
    impl ServerStatus{
        pub fn new()->ServerStatus{
            ServerStatus{
                status:0,
                version:VERSION.to_string(),
                data:ImageBucket{
                    count:0,
                    last_update:mongodb::bson::DateTime::from_millis(0).to_string(),
                    api_call_count:0,
                    traffic:0
                }
            }
        }
    
    }
}