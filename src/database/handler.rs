pub mod handler{
    use mongodb::{Client, bson, Collection};
    use mongodb::bson::{doc, Document};
    use futures::stream::StreamExt;
    use log::info;
    use crate::entity::image_detail::image_detail::ImageDetail;
    use crate::entity::config::config::{ CONFIG};

    static mut CLIENT:Option<Client> = None;
    pub async fn image_count() -> Option<u64>{
        unsafe{
            if CLIENT.is_none(){
                CLIENT = Some(Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str()).await.unwrap());
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
                        doc!{
                            "$count":"source"
                        }
                    ];
                    let mut cur = col.aggregate(pipeline,None).await.unwrap();
                    if let Some(result) = cur.next().await{
                        match result {
                            Ok(document)=>{
                                return Some(document.get_i32("source").unwrap_or_default() as u64);
                            },
                            Err(_)=>{}
                        }

                    }
                }
                None => {return None;},
            }
            return Some(0)
        }
        
    }

    pub async fn last_time() -> bson::DateTime{
        unsafe{
            if CLIENT.is_none(){
                CLIENT = Some(Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str()).await.unwrap());
            }
            match &CLIENT {
                Some(client) => {
                    let db = client.database("anime");
                    let col: Collection<Document> = db.collection("artwork");
                    let pipeline = vec![
                        doc!{
                            "$sort":{
                                "created_at": -1i32
                            }
                        },
                        doc!{
                            "$limit":1i32
                        }
                    ];
                    let mut cur = col.aggregate(pipeline,None).await.unwrap();
                    if let Some(result) = cur.next().await{
                        match result {
                            Ok(document)=>{
                                let ts = document.get_i32("created_at").unwrap() as i64;
                                return bson::DateTime::from_millis(ts*1000);
                            },
                            Err(_)=>{return bson::DateTime::from_millis(0);}
                        }
                    }
                },
                None => {return bson::DateTime::from_millis(0);},
            }
            return bson::DateTime::from_millis(0);
        }
    }

    pub async fn sample_one(id:Option<&String>, nin_tags:Option<Vec<&str>>, horizontal:Option<bool>) -> Option<ImageDetail> {
        let mut image = None;
        unsafe {
            if CLIENT.is_none(){
                CLIENT = Some(Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str()).await.unwrap());
            }
            let mut cursor;
            match &CLIENT {
                None => {}
                Some(client) => {
                    let db = client.database("anime");
                    let col = db.collection("artwork");
                    if id.is_some(){
                        let find = doc!{
                                "_id":id.unwrap().parse::<u32>().unwrap()
                        };
                        cursor = col.find(find, None).await.unwrap();

                    }else{
                        let mut nin = vec!["nipples","nude","pussy","pussy_juice","uncensored","breasts"];
                        if nin_tags.is_some() {
                            nin.extend(nin_tags.unwrap().iter());
                        }
                        let mut pipeline = vec![
                            doc!{
                                "$match":{
                                    "file_url":{"$regex":"(jpg|png)$"},
                                    "created_at":{"$gt":1506787200},
                                    "file_size":{"$lt":10*1024*1024},
                                    "file_size":{"$gt":500*1024},
                                    "rating_on_ml":"s"
                                }
                            }
                        ];
                        match horizontal {
                            Some(hor) => {
                                let hor_value;
                                if hor {
                                    hor_value = doc! {"$expr":{"$gt":["jpeg_width","jpeg_height"]}};
                                }else{
                                    hor_value = doc! {"$expr":{"$gt":["jpeg_height","jpeg_width"]}};
                                }
                                pipeline.push(hor_value);
                            },
                            None => {},
                        }
                        pipeline.push(doc!{
                            "$sample":{
                                "size":1
                            }
                        });
                        cursor = col.aggregate(pipeline,None).await.unwrap();
                    }

                    if let Some(result) = cursor.next().await{
                        match result {
                            Ok(document)=>{
                                image = Some(bson::from_document(document).unwrap());
                                info!("{:?}",&image);
                            },
                            Err(_)=>{}
                        }

                    }
                }
            }

        }
        image

    }
}
