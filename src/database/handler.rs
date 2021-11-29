pub mod handler{
    use mongodb::{Client, bson};
    use mongodb::bson::doc;
    use futures::stream::StreamExt;
    use log::info;
    use crate::entity::image_detail::image_detail::ImageDetail;
    use crate::entity::config::config::{ CONFIG};

    static mut CLIENT:Option<Client> = None;
    pub async fn sample_one(id:Option<&String>, nin_tags:Option<Vec<&str>>) -> Option<ImageDetail> {
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
                        let pipeline = vec![
                            doc!{
                            "$match":{
                                "file_url":{"$regex":"(jpg|png)$"},
                                "created_at":{"$gt":1506787200},
                                "file_size":{"$lt":10*1024*1024},
                                "file_size":{"$gt":500*1024},
                                "rating_on_ml":"s"}
                            }
                        },
                            doc!{
                            "$sample":{
                                "size":1
                            }
                        }
                        ];
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
