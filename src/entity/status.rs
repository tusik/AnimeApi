/*
 * @Author: Image_woker_pc image@by.cx
 * @Date: 2022-08-08 15:37:18
 * @LastEditors: Image_woker_pc image@by.cx
 * @LastEditTime: 2022-10-08 14:06:44
 * @FilePath: \AnimeApi\src\entity\status.rs
 * @filePathColon: /
 * @Description: 
 * 
 * Copyright (c) 2022 by Image_woker_pc image@by.cx, All Rights Reserved. 
 */
pub mod status{
    use serde::{Serialize, Deserialize};

    #[derive(Debug,Serialize,Deserialize)]
    pub struct ImageBucket{
        pub count : u64,
        pub last_update: String,
        pub api_call_count:u64
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct ServerStatus{
        pub status: u64,
        pub data:ImageBucket
        
    }
    impl ServerStatus{
        pub fn new()->ServerStatus{
            ServerStatus{
                status:0,
                data:ImageBucket{
                    count:0,
                    last_update:mongodb::bson::DateTime::from_millis(0).to_string(),
                    api_call_count:0
                }
            }
        }
    
    }
}