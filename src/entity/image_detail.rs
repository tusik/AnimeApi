pub mod image_detail{
    use serde::Serialize;
    use serde::Deserialize;
    #[derive(Debug,Serialize,Deserialize)]
    pub struct ImageDetail{
        pub(crate) file_url:String,
        pub(crate) md5:String
    }
}
