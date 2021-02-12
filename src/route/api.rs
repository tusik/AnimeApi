#[allow(dead_code)]
pub mod api{
    use crate::database::handler::handler::sample_one;
    use warp::{Filter, Rejection};
    use warp::http::{Response};
    use std::collections::HashMap;
    use crate::entity::config::config::{ CONFIG};

    pub fn api_sample() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::any()
            .and(warp::path("image"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image)
    }
    pub fn read_img(){

    }
    pub async fn sample_image(params: HashMap<String, String>) -> Result<Response<Vec<u8>>, Rejection> {
        let mut nin:Option<Vec<&str>> = None;
        match params.get("nin"){
            None => {}
            Some(item) => {
                nin = Some(item.split(",").collect());
            }
        }
        let image = sample_one(params.get("id"),nin).await.unwrap();
        let md5 = image.md5;
        let tmp_string:Vec<&str> = image.file_url.split("/").collect();
        let ext_tmp:Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        let mut path_prefix ="";
        unsafe {
            path_prefix =  &CONFIG.unwrap().system.path;
        }
        let mut full_name = format!("{}/images_opt/{}/{}_optmized.{}",path_prefix, &md5[0..2], &md5, ext);
        match params.get("size"){
            None => {}
            Some(mode) => match mode.as_ref(){
                "preview"=>{
                    if image.width > 150{
                        full_name = format!("{}/images_preview/{}/{}_optmized.{}",path_prefix,&md5[0..2],&md5,ext);
                    }
                }
                "middle"=>{
                    if image.width > 1500 {
                        full_name = format!("{}/images_middle/{}/{}_optmized.{}",path_prefix, &md5[0..2], &md5, ext);
                    }
                }
                &_ => {}
            }
        }
        let img_data = tokio::fs::read(full_name).await.unwrap();
        let mut content_type = "image/jpeg";
        if *ext=="png"{
            content_type = "image/png";
        }else if *ext=="jpg"{
            content_type = "image/jpeg";
        }
        let resp = Response::builder()
            .header("content-type",content_type)
            .header("image_id",image._id)
            .header("image_source",image.source)
            .header("image_tags",image.tags.join(","))
            .body(img_data).unwrap();
        Ok(resp)
    }
}