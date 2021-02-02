pub mod handler{
    use mongodb::{Client, bson};
    use mongodb::bson::doc;
    use futures::stream::StreamExt;
    use crate::entity::image_detail::image_detail::ImageDetail;
    use crate::entity::config::config::{ CONFIG};

    static mut CLIENT:Option<Client> = None;
    pub async fn sample_one() -> Option<ImageDetail> {
        let mut image = None;
        unsafe {
            if CLIENT.is_none(){
                CLIENT = Some(Client::with_uri_str(CONFIG.as_ref().unwrap().system.mongo_uri.as_str()).await.unwrap());
            }
            match &CLIENT {
                None => {}
                Some(client) => {
                    let db = client.database("anime");
                    let col = db.collection("artwork");
                    let pipeline = vec![
                        doc!{
                            "$match":{
                                "file_url":{"$regex":"(jpg|png)$"},
                                "created_at":{"$gt":1420041600},
                                "rating":"s",
                                "file_size":{"$lt":5*1024*1024}
                            }
                        },
                        doc!{
                            "$sample":{
                                "size":1
                            }
                        }
                    ];
                    let mut cursor = col.aggregate(pipeline,None).await.unwrap();
                    if let Some(result) = cursor.next().await{
                        match result {
                            Ok(document)=>{
                                image = Some(bson::from_document(document).unwrap());
                                println!("{:?}",&image);
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