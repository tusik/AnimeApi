pub mod image_detail {
    use serde::Deserialize;
    use serde::Serialize;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageDetail {
        pub(crate) file_url: String,
        pub(crate) file_size: usize,
        pub(crate) md5: String,
        pub(crate) tags: Vec<String>,
        pub(crate) width: u32,
        pub(crate) height: u32,
        pub(crate) source: String,
        pub(crate) author: String,
        pub(crate) has_children: bool,
        pub(crate) _id: u32,
    }
    impl ImageDetail {
        pub fn ext(&self) -> String {
            self.file_url
                .split(".")
                .last()
                .unwrap_or_default()
                .to_string()
        }
    }
}
