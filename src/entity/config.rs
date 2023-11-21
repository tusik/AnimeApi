
pub mod config{
    use crate::util::ipcheck::checker::Checker;
    use crate::util::ipcheck::checker::Country;
    use serde::Serialize;
    use serde::Deserialize;
    pub static mut CONFIG:Option<Config> = None;
    pub static mut CHECKER:Option<Checker> = None;
    #[derive(Debug,Serialize,Deserialize)]
    pub struct Config{
        pub system:SystemConfig,
        pub host:HostConfig
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct SystemConfig{
        pub mongo_uri:String,
        pub path:String,
        pub origin_img:String,
        pub preview_img:String
    }
    #[derive(Debug,Serialize,Deserialize)]
    pub struct HostConfig{
        pub domain:Vec<String>,
        pub redis:String
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
            let mut checker = Checker::new();
            unsafe {
                checker.read_ip(Country::CN);
                CHECKER = Some(checker);
            }
        }
    }
}
