pub mod image_detail{
    use serde::Serialize;
    use serde::Deserialize;
    #[derive(Debug,Serialize,Deserialize)]
    pub struct ImageDetail{
        pub(crate) file_url:String,
        pub(crate) md5:String,
        pub(crate) tags:Vec<String>,
        pub(crate) width:u32,
        pub(crate) height:u32,
        pub(crate) source:String,
        pub(crate) author:String,
        pub(crate) has_children:bool,
        pub(crate) _id:u32
    }
}
