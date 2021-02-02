pub mod config{
    use serde::Serialize;
    use serde::Deserialize;
    pub static mut CONFIG:Option<SystemConfig> = None;
    #[derive(Debug,Serialize,Deserialize)]
    pub struct SystemConfig{
        pub(crate) mongo_uri:String
    }
    impl SystemConfig{
        pub async fn load() {
            let config_file = tokio::fs::read_to_string("./config.toml").await;
            let c:SystemConfig;
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