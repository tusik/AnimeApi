#[allow(dead_code)]
pub mod api{
    use crate::database::handler::handler::sample_one;
    use warp::{Filter, Rejection};
    use warp::http::{Response, Uri};
    use std::collections::HashMap;
    use crate::entity::config::config::{ CONFIG};
    use crate::entity::image_detail::image_detail::ImageDetail;
    use serde_json;
    use std::str::FromStr;

    pub fn api_sample() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::get()
            .and(warp::path("image"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image)
    }

    pub fn api_sample_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::post()
            .and(warp::path("image"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_post)
    }
    pub fn api_sample_red_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::get()
            .and(warp::path("images"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_redirect)
    }
    pub fn api_sample_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::get()
            .and(warp::path("images.json"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_post)
    }
    pub async fn read_image(img_data: &mut Vec<u8>, image:&ImageDetail, size:Option<&String>) -> Result<usize,usize>{
        let md5 = image.md5.clone();
        let tmp_string:Vec<&str> = image.file_url.split("/").collect();
        let ext_tmp:Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        let path_prefix;
        let origin_prefix;
        let preview_prefix;
        unsafe {
            path_prefix =  CONFIG.as_ref().unwrap().system.path.as_str();
            origin_prefix =  CONFIG.as_ref().unwrap().system.origin_img.as_str();
            preview_prefix =  CONFIG.as_ref().unwrap().system.preview_img.as_str();
        }
        let mut full_name = format!("{}/{}/{}/{}.{}",&path_prefix,&origin_prefix, &md5[0..2], &md5, ext);
        match size{
            Some(s)=> match s.as_str(){
                "preview"=>{
                    if image.width > 150{
                        full_name = format!("{}/{}/{}/{}_optmized.{}",&path_prefix,&preview_prefix,&md5[0..2],&md5,ext);
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
                img_data.extend_from_slice(d.as_slice());
                return Ok(img_data.len());
            },
            _ => {}
        }
        return Err(0);
    }
    pub fn get_content_type(ext:&str) -> &'static str {
        if ext=="png"{
            return "image/png";
        }else if ext=="jpg"{
            return "image/jpeg";
        }else{
            return "image/jpeg";
        }
    }
    pub fn get_file_ext(filename:&str) -> &str {
        let tmp_string:Vec<&str> = filename.split("/").collect();
        let ext_tmp:Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        return ext.clone();
    }
    pub async fn sample_image_post(params: HashMap<String, String>) -> Result<Response<String>, Rejection> {
        let mut nin:Option<Vec<&str>> = None;
        match params.get("nin"){
            None => {}
            Some(item) => {
                nin = Some(item.split(",").collect());
            }
        }
        let direction = match params.get("direction") {
            Some(v) => Some(v == "horizontal"),
            None => None,
        };
        let image = sample_one(params.get("id"),nin, direction).await.unwrap();
        let resp = Response::builder()
            .header("content-type","application/json")
            .body(serde_json::to_string(&image).unwrap()).unwrap();
        Ok(resp)
    }
    pub async fn sample_image(params: HashMap<String, String>) -> Result<Response<Vec<u8>>, Rejection> {
        let mut nin:Option<Vec<&str>> = None;
        match params.get("nin"){
            None => {}
            Some(item) => {
                nin = Some(item.split(",").collect());
            }
        }
        let direction = match params.get("direction") {
            Some(v) => Some(v == "horizontal"),
            None => None,
        };
        let image = sample_one(params.get("id"),nin, direction).await.unwrap();

        let ext = get_file_ext(image.file_url.as_str());

        let mut img_data = vec![];

        while read_image(&mut img_data, &image,params.get("size")).await.is_err() {}

        let content_type= get_content_type(ext);
        let resp = Response::builder()
            .header("content-type",content_type)
            .header("image_id",image._id)
            .header("image_source",image.source)
            .header("image_tags",image.tags.join(","))
            .body(img_data).unwrap();
        Ok(resp)
    }
    pub async fn sample_image_redirect(params: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
        let mut nin:Option<Vec<&str>> = None;
        match params.get("nin"){
            None => {}
            Some(item) => {
                nin = Some(item.split(",").collect());
            }
        }
        let direction = match params.get("direction") {
            Some(v) => Some(v == "horizontal"),
            None => None,
        };
        let image = sample_one(params.get("id"), nin, direction).await.unwrap();

        let ext = get_file_ext(image.file_url.as_str());
        let mut  target_link:String = "https://b2.pic.re/".to_string();
        target_link+= &image.md5[0..2];
	    target_link+= "/";
        target_link+= &image.md5;
	    target_link+=".";
        target_link+=ext;
        Ok(warp::redirect::temporary(Uri::from_str(target_link.as_str()).unwrap()))
    }
}
