pub mod config{
    use serde::Serialize;
    use serde::Deserialize;
    pub static mut CONFIG:Option<Config> = None;
    #[derive(Debug,Serialize,Deserialize)]
    pub struct Config{
        pub system:SystemConfig
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct SystemConfig{
        pub mongo_uri:String,
        pub path:String
    }
    impl Config{
        pub async fn load() {
            let config_file = tokio::fs::read_to_string("./config.toml").await;
            let c:Config;
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