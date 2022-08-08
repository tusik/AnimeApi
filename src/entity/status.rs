pub mod status{
    use serde::{Serialize, Deserialize};

    #[derive(Debug,Serialize,Deserialize)]
    pub struct ImageBucket{
        pub size : u64,
        pub lastUpdate: mongodb::bson::DateTime,
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
                    size:0,
                    lastUpdate:mongodb::bson::DateTime::from_millis(0),
                }
            }
        }
    
    }
}