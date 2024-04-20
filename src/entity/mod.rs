use serde::{Deserialize, Serialize};

pub mod config;
pub mod image_detail;
pub mod status;
pub mod condition;

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub count: u32,
}
