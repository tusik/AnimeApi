pub mod handler {
    use std::collections::HashMap;
    use std::time::SystemTime;
    use std::vec;

    use crate::entity::config::config::CONFIG;
    use crate::entity::image_detail::image_detail::ImageDetail;
    use crate::entity::Tag;
    use futures::stream::StreamExt;
    use mongodb::bson::{doc, Document};
    use mongodb::{bson, Client, Collection};
    extern crate redis;
    use redis::Commands;
    static mut CLIENT: Option<Client> = None;
    static mut REDIS_CLIENT: Option<redis::Client> = None;

    pub fn get_redis() -> &'static Option<redis::Client> {
        let redis_host;
        let r_client;
        unsafe {
            redis_host = CONFIG.as_ref().unwrap().host.redis.as_str();
            if REDIS_CLIENT.is_none() {
                REDIS_CLIENT = Some(redis::Client::open(redis_host).expect("unable to open"));
            }
            r_client = &REDIS_CLIENT;
        }
        r_client
    }
    pub fn redis_incr() {
        let r_client = get_redis();
        match r_client {
            Some(c) => {
                let mut con = c.get_connection().expect("Unable to connecet redis");
                let _: () = con.incr("cv", 1).expect("incr count failed");
            }
            None => {}
        }
    }
    pub fn redis_incr_key(key: &str, value: usize) {
        let r_client = get_redis();
        match r_client {
            Some(c) => {
                let mut con = c.get_connection().expect("Unable to connecet redis");
                let _: () = con.incr(key, value).expect("incr count failed");
            }
            None => {}
        }
    }
    pub fn redis_get_value(key: &str) -> Option<u64> {
        let r_client = get_redis();
        match r_client {
            Some(c) => {
                let mut con = c.get_connection().expect("Unable to connecet redis");
                let v = con.get(key).expect("failed get redis value");
                v
            }
            None => None,
        }
    }
    pub fn call_count() -> Option<u64> {
        let r_client = get_redis();
        match r_client {
            Some(c) => {
                let mut con = c.get_connection().expect("Unable to connecet redis");
                let cv = con.get("cv").expect("failed get cv");
                cv
            }
            None => None,
        }
    }

    pub async fn image_count() -> Option<u64> {
        unsafe {
            if CLIENT.is_none() {
                CLIENT = Some(
                    Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str())
                        .await
                        .unwrap(),
                );
            }
            match &CLIENT {
                Some(client) => {
                    let db = client.database("anime");
                    let col: Collection<Document> = db.collection("artwork");
                    let pipeline = vec![
                        doc! {
                            "$match":{
                                "rating_on_ml":"s"
                            }
                        },
                        doc! {
                            "$count":"source"
                        },
                    ];
                    let mut cur = col.aggregate(pipeline, None).await.unwrap();
                    if let Some(result) = cur.next().await {
                        match result {
                            Ok(document) => {
                                return Some(document.get_i32("source").unwrap_or_default() as u64);
                            }
                            Err(_) => {}
                        }
                    }
                }
                None => {
                    return None;
                }
            }
            return Some(0);
        }
    }

    pub async fn last_time() -> bson::DateTime {
        unsafe {
            if CLIENT.is_none() {
                CLIENT = Some(
                    Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str())
                        .await
                        .unwrap(),
                );
            }
            match &CLIENT {
                Some(client) => {
                    let db = client.database("anime");
                    let col: Collection<Document> = db.collection("artwork");
                    let pipeline = vec![
                        doc! {
                            "$sort":{
                                "created_at": -1i32
                            }
                        },
                        doc! {
                            "$limit":1i32
                        },
                    ];
                    let mut cur = col.aggregate(pipeline, None).await.unwrap();
                    if let Some(result) = cur.next().await {
                        match result {
                            Ok(document) => {
                                let ts = document.get_i32("created_at").unwrap() as i64;
                                return bson::DateTime::from_millis(ts * 1000);
                            }
                            Err(_) => {
                                return bson::DateTime::from_millis(0);
                            }
                        }
                    }
                }
                None => {
                    return bson::DateTime::from_millis(0);
                }
            }
            return bson::DateTime::from_millis(0);
        }
    }

    pub async fn sample_one(
        id: Option<&String>,
        nin_tags: Option<Vec<&str>>,
        horizontal: Option<bool>,
        params: HashMap<String, String>,
    ) -> Option<ImageDetail> {
        let start_time = SystemTime::now();
        redis_incr();
        let mut image = None;
        unsafe {
            if CLIENT.is_none() {
                CLIENT = Some(
                    Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str())
                        .await
                        .unwrap(),
                );
            }
            let mut cursor;
            match &CLIENT {
                None => {}
                Some(client) => {
                    let db = client.database("anime");
                    let col = db.collection("artwork");
                    if id.is_some() {
                        let find = doc! {
                                "_id":id.unwrap().parse::<u32>().unwrap()
                        };
                        cursor = col.find(find, None).await.unwrap();
                    } else {
                        let mut nin = vec![
                            "nipples",
                            "nude",
                            "pussy",
                            "pussy_juice",
                            "uncensored",
                            "breasts",
                        ];
                        if nin_tags.is_some() {
                            nin.extend(nin_tags.unwrap().iter());
                        }

                        let min = match params.get("min_size") {
                            Some(v) => v.parse::<u32>().unwrap_or(640),
                            None => 640,
                        };
                        let max = match params.get("max_size") {
                            Some(v) => v.parse::<u32>().unwrap_or(6144),
                            None => 6144,
                        };
                        let mut pipeline = vec![doc! {
                            "$match":{
                                "rating_on_ml":"s",
                                "created_at":{"$gt":1506787200},
                                "file_size":{"$gt":500*1024, "$lt":12*1024*1024},
                                "$and":[
                                    {"jpeg_width":{"$gt":min, "$lt":max}},
                                    {"jpeg_height":{"$gt":min, "$lt":max}}
                                ]
                            }
                        }];
                        match horizontal {
                            Some(hor) => {
                                let hor_value;
                                if hor {
                                    hor_value = doc! {"$expr":{"$gt":["jpeg_width","jpeg_height"]}};
                                } else {
                                    hor_value = doc! {"$expr":{"$gt":["jpeg_height","jpeg_width"]}};
                                }
                                pipeline.push(hor_value);
                            }
                            None => {}
                        }
                        pipeline.push(doc! {
                            "$sample":{
                                "size":1
                            }
                        });
                        cursor = col.aggregate(pipeline, None).await.unwrap();
                    }

                    if let Some(result) = cursor.next().await {
                        match result {
                            Ok(document) => {
                                let mut res: ImageDetail = bson::from_document(document).unwrap();
                                res.file_url = CONFIG.as_ref().unwrap().host.domain[0].clone()
                                    + "/"
                                    + &res.md5[0..2]
                                    + "/"
                                    + &res.md5
                                    + "."
                                    + &res.ext();
                                image = Some(res);
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
        }
        println!(
            "mongo time: {:?}",
            SystemTime::now().duration_since(start_time).unwrap()
        );
        image
    }
    pub async fn get_tags() -> Vec<Tag> {
        unsafe {
            if CLIENT.is_none() {
                CLIENT = Some(
                    Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str())
                        .await
                        .unwrap(),
                );
            }
            let mut cursor;
            match &CLIENT {
                None => {
                    vec![]
                }
                Some(client) => {
                    let db = client.database("anime");
                    let col: Collection<Document> = db.collection("artwork");
                    let pipeline = vec![
                        doc! {
                            "$unwind":"$tags"
                        },
                        doc! {
                            "$group":{
                                "_id":"$tags",
                                "count":{"$sum":1}
                            }
                        },
                        doc! {
                            "$match":{
                                "count":{"$gt":2}
                            }
                        },
                        doc! {
                            "$sort":{
                                "count":-1
                            }
                        },
                    ];
                    cursor = col.aggregate(pipeline, None).await.unwrap();
                    let mut tags = vec![];
                    while let Some(result) = cursor.next().await {
                        match result {
                            Ok(document) => {
                                let tag = document.get_str("_id").unwrap_or_default();
                                let count = document.get_i32("count").unwrap_or_default() as u32;
                                tags.push(Tag {
                                    name: tag.to_string(),
                                    count: count,
                                });
                            }
                            Err(_) => {}
                        }
                    }
                    return tags;
                }
            }
        }
    }
}
