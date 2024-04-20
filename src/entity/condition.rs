use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use warp::reject::Reject;
use crate::entity::condition::SearchError::ConditionParseError;

#[derive(Default,Clone)]
pub struct SearchCondition{
    pub id:Option<u32>,
    pub exclude_tags:Option<Vec<String>>,
    pub include_tags:Option<Vec<String>>,
    pub horizontal:Option<bool>,
    pub compress: Option<bool>,
    pub min_size:u32,
    pub max_size:u32
}
pub enum SearchError{
    ConditionParseError,
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SearchError::ConditionParseError => write!(f, "search condition parse error"),

        }
    }
}

impl Debug for SearchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            SearchError::ConditionParseError => write!(f, "search condition parse error"),

        }
    }
}

impl Reject for SearchError {
    
}
impl std::error::Error for SearchError {}
impl SearchCondition{
    pub fn default()->Self{
        SearchCondition{
            id: None ,
            exclude_tags: None,
            include_tags: None,
            horizontal: None,
            compress: Some(true),
            min_size: 640,
            max_size: 6144,
        }
    }
    pub fn parse(params:HashMap<String,String>)->Result<SearchCondition,SearchError>{
        let mut condition = SearchCondition::default();
        match params.get("id") {
            None => {}
            Some(id) => {
                condition.id=match id.parse::<u32>() {
                    Ok(id) => {Some(id)}
                    Err(_) => {return Err(SearchError::ConditionParseError);}
                }
            }
        }
        match params.get("nin") {
            None => {}
            Some(item) => {
                condition.exclude_tags = Some(item.split(',')
                    .map(|s| s.to_string())
                    .collect());
            }
        }
        match params.get("in") {
            None => {}
            Some(item) => {
                condition.include_tags = Some(item.split(',')
                    .map(|s| s.to_string())
                    .collect());
            }
        }
        match params.get("compress") {
            None => {}
            Some(compress) => {
                condition.compress = match compress.parse::<bool>() {
                    Ok(c) => {Some(c)}
                    Err(_) => {return Err(ConditionParseError)}
                }
            }
        }
        condition.horizontal = match params.get("direction") {
            Some(v) => Some(v == "horizontal"),
            None => None,
        };
        Ok(condition)
    }
}