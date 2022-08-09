pub mod status{
    use serde::{Serialize, Deserialize};

    #[derive(Debug,Serialize,Deserialize)]
    pub struct ImageBucket{
        pub count : u64,
        pub last_update: String,
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct ServerStatus{
        pub status: u64,
        pub data:ImageBucket,
        
    }
    impl ServerStatus{
        pub fn new()->ServerStatus{
            ServerStatus{
                status:0,
                data:ImageBucket{
                    count:0,
                    last_update:mongodb::bson::DateTime::from_millis(0).to_string(),
                }
            }
        }
    
    }
}