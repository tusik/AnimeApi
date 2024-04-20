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
    use tokio::sync::{Mutex, OnceCell};
    use std::sync::Arc;
    use futures::TryStreamExt;
    use crate::entity::condition::SearchCondition;

    static CLIENT:OnceCell<Arc<Client>> = OnceCell::const_new();
    static REDIS_CLIENT: OnceCell<Arc<Mutex<redis::Client>>> = OnceCell::const_new();

    static QUERY_CACHE: OnceCell<Arc<Mutex<HashMap<String, Vec<ImageDetail>>>>> = OnceCell::const_new();
    pub fn init_query_cache(){
        let cache = Arc::new(Mutex::new(HashMap::new()));
        QUERY_CACHE.set(cache).ok();
    }
    pub async fn init_mongo(){
        let config = CONFIG.get().expect("failed mongo client");
        let uri = &config.clone().system.mongo_uri;
        let client = Client::with_uri_str(uri)
            .await
            .unwrap();
        CLIENT.set(Arc::new(client)).expect("")
    }
    pub fn init_redis(){
        let config = CONFIG.get().expect("");
        let dev = config.system.dev;
        let host = config.host.clone().redis;
        if dev{
            return;
        }
        let client = Arc::new(Mutex::new(redis::Client::open(host.as_str()).expect("connect redis failed")));
        REDIS_CLIENT.set(client.clone()).expect("Failed to set Redis client");
    }
    pub fn get_redis() -> Arc<Mutex<redis::Client>> {
        REDIS_CLIENT.get().cloned().expect("Redis client is not initialized")
    }
    pub async fn redis_incr() {
        let config = CONFIG.get().expect("");
        if config.system.dev{
            return;
        }

        let r_client = get_redis();
        let mut con = r_client.lock().await.get_connection().expect("Unable to connecet redis");
        let _: () = con.incr("cv", 1).expect("incr count failed");
    }
    pub async fn redis_incr_key(key: &str, value: usize) {
        let config = CONFIG.get().expect("");
        if config.system.dev{
            return;
        }
        let r_client = get_redis();
        let mut con = r_client.lock().await.get_connection().expect("Unable to connecet redis");
        let _: () = con.incr(key, value).expect("incr count failed");

    }
    pub async fn redis_get_value(key: &str) -> Option<u64> {
        let r_client = get_redis();
        let mut con = r_client.lock().await.get_connection().expect("Unable to connecet redis");
        let v = con.get(key).expect("failed get redis value");
        v

    }
    pub async fn call_count() -> Option<u64> {
        let r_client = get_redis();
        let mut con = r_client.lock().await.get_connection().expect("Unable to connecet redis");
        let cv = con.get("cv").expect("failed get cv");
        cv
    }
    pub async fn get_client() -> Option<Arc<Client>> {
        Some(Arc::clone(CLIENT.get().expect("")))
    }
    pub async fn image_count() -> Option<u64> {
        match &get_client().await {
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

    pub async fn last_time() -> bson::DateTime {
        match &get_client().await {
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
                    return match result {
                        Ok(document) => {
                            let ts = document.get_i32("created_at").unwrap() as i64;
                            bson::DateTime::from_millis(ts * 1000)
                        }
                        Err(_) => {
                            bson::DateTime::from_millis(0)
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

    pub async fn sample_one(
        search_condition: SearchCondition
    ) -> Option<ImageDetail> {
        let start_time = SystemTime::now();
        redis_incr().await;
        let mut image = None;
        let mut cursor = None;
        match &get_client().await {
            None => {}
            Some(client) => {
                let db = client.database("anime");
                let col = db.collection("artwork");
                if search_condition.id.is_some() {
                    let find = doc! {
                            "_id":search_condition.id
                    };
                    cursor = Some(col.find(find, None).await.unwrap());
                } else {
                    let mut nin:Vec<String> = vec![];
                    if search_condition.exclude_tags.is_some() {
                        nin.extend(search_condition.exclude_tags.unwrap());
                    }

                    let mut pipeline = vec![doc! {
                        "$match":{
                            "rating_on_ml":"s",
                            "created_at":{"$gt":1506787200},
                            "file_size":{"$gt":500*1024, "$lt":12*1024*1024},
                            "$and":[
                                {"jpeg_width":{"$gt":search_condition.min_size, "$lt":search_condition.max_size}},
                                {"jpeg_height":{"$gt":search_condition.min_size, "$lt":search_condition.max_size}}
                            ]
                        }
                    }];
                    match search_condition.horizontal {
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
                            "size":100
                        }
                    });

                    let pipeline_str = serde_json::to_string(&pipeline).unwrap();
                    {
                        let cache =QUERY_CACHE.get().cloned().expect("QUERY_CACHE is not initialized");
                        let mut cache = cache.lock().await;

                        // 尝试获取给定key的缓存
                        let entry = cache.entry(pipeline_str.clone()).or_insert_with(Vec::new);

                        if let Some(image_detail) = entry.pop() {
                            // 如果能从vec中移除一个项，则对`image`变量赋值
                            image = Some(image_detail);
                        } else {
                            // 如果vec为空或不存在，则执行聚合查询
                            match col.aggregate(pipeline, None).await {
                                Ok(cur) => cursor = Some(cur),
                                Err(e) => {
                                    eprintln!("Error executing aggregate: {:?}", e);
                                    return None;
                                }
                            }
                        }

                    }
                    if image.is_none() {
                        if let Some(mut cur) = cursor {
                            while let Ok(result) = cur.try_next().await {
                                match result {
                                    Some(document) => {
                                        let image_detail = match bson::from_document(document) {
                                            Ok(detail) => detail,
                                            Err(e) => {
                                                eprintln!("Error converting BSON to ImageDetail: {:?}", e);
                                                continue; // 跳过该项，处理下一项
                                            }
                                        };

                                        if image.is_none() {
                                            image = Some(image_detail);
                                        } else {
                                            let cache =QUERY_CACHE.get().cloned().expect("QUERY_CACHE is not initialized");
                                            let mut cache = cache.lock().await;
                                            // 确保不会与其他任务冲突地更新缓存
                                            let entry = cache.entry(pipeline_str.clone()).or_insert_with(Vec::new);
                                            entry.push(image_detail);

                                        }
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        println!(
            "mongo time: {:?}",
            SystemTime::now().duration_since(start_time).unwrap()
        );
        let config = CONFIG.get()?;
        let domain = &config.host.domain[0].clone();
        let mut i = image.unwrap();
        i.file_url = domain.as_str().to_owned()
            + "/"
            + &i.md5[0..2]
            + "/"
            + &i.md5
            + "."
            + &i.ext();
        image = Some(i);


        image
    }
    pub async fn get_tags() -> Vec<Tag> {
        let mut cursor;
        match &get_client().await {
            None => {
                vec![]
            }
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
                            "count":{"$gt":10}
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
