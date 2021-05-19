#[allow(dead_code)]
pub mod api{
    use crate::database::handler::handler::sample_one;
    use warp::{Filter, Rejection};
    use warp::http::{Response};
    use std::collections::HashMap;
    use crate::entity::config::config::{ CONFIG};
    use std::io;
    use crate::entity::image_detail::image_detail::ImageDetail;

    pub fn api_sample() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::get()
            .and(warp::path("image"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image)
    }

    pub async fn read_image(img_data: &mut Vec<u8>, image:&ImageDetail, size:Option<&String>) -> Result<usize,usize>{
        let md5 = image.md5.clone();
        let tmp_string:Vec<&str> = image.file_url.split("/").collect();
        let ext_tmp:Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        let mut path_prefix;
        unsafe {
            path_prefix =  CONFIG.as_ref().unwrap().system.path.as_str();
        }
        let mut full_name = format!("{}/images_opt/{}/{}_optmized.{}",&path_prefix, &md5[0..2], &md5, ext);
        match size{
            Some(s)=> match s.as_str(){
                "preview"=>{
                    if image.width > 150{
                        full_name = format!("{}/images_preview/{}/{}_optmized.{}",&path_prefix,&md5[0..2],&md5,ext);
                    }
                }
                "middle"=>{
                    if image.width > 1500 {
                        full_name = format!("{}/images_middle/{}/{}_optmized.{}",&path_prefix, &md5[0..2], &md5, ext);
                    }
                }
                &_ => {}
            }
            _ => {}
        }
        let img_res =  tokio::fs::read(full_name).await;
        match &img_res {
            Ok(d)=>{
                img_data.clone_from_slice(d.as_slice());
                return Ok(img_data.len());
            },
            _ => {}
        }
        return Err(0);
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
        let tmp_string:Vec<&str> = image.file_url.split("/").collect();
        let ext_tmp:Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        let mut img_data = vec![];

        while read_image(&mut img_data, &image,params.get("size")).await.is_err() {

        }

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