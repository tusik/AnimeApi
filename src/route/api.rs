
#[allow(dead_code)]
pub mod api {
    use crate::database::handler::handler::{
        call_count, get_tags, image_count, last_time, redis_get_value, redis_incr_key, sample_one,
    };
    use crate::entity::config::config::CONFIG;
    use crate::entity::image_detail::image_detail::ImageDetail;
    use crate::entity::status::status::ServerStatus;
    use serde_json;
    use std::collections::HashMap;
    use std::time::SystemTime;
    use warp::{http::Response, Filter, Rejection, hyper};
    use crate::entity::condition::SearchCondition;

    pub fn api_sample() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::get()
            .and(warp::path("image"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image)
    }

    pub fn api_sample_post(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("image"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_post)
    }
    pub fn api_sample_red_post(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path("images"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_redirect)
    }
    pub fn api_sample_json(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path("image.json"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_post)
    }
    pub fn api_sample_json_post(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::post()
            .and(warp::path("image.json"))
            .and(warp::query::<HashMap<String, String>>())
            .and_then(sample_image_post)
    }
    pub fn api_server_status(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get()
            .and(warp::path("status"))
            .and_then(server_status)
    }
    pub fn api_image_cors(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::options().and(warp::path("image")).and_then(cors)
    }
    pub fn api_images_cors(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::options().and(warp::path("images")).and_then(cors)
    }
    pub fn api_imagejson_cors(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::options().and(warp::path("image.json")).and_then(cors)
    }
    pub fn api_tags() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::get().and(warp::path("tags")).and_then(tags)
    }
    pub async fn cors() -> Result<Response<String>, Rejection> {
        let resp = Response::builder()
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "*")
            .body("".to_string())
            .unwrap();
        Ok(resp)
    }
    pub async fn tags() -> Result<String, Rejection> {
        Ok(serde_json::to_string(&get_tags().await).unwrap())
    }
    pub async fn server_status() -> Result<Response<String>, Rejection> {
        let mut status = ServerStatus::new();
        let call_count = call_count().await;
        let size = image_count().await;
        let traffic = redis_get_value("traffic").await;
        status.data.count = size.unwrap_or(0);
        status.data.last_update = last_time().await.to_string();
        status.data.api_call_count = call_count.expect("fetch call count failed");
        status.data.traffic = traffic.unwrap_or(0);
        let resp = Response::builder()
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&status).unwrap_or("".to_string()))
            .unwrap();
        Ok(resp)
    }
    pub async fn read_image(
        img_data: &mut Vec<u8>,
        image: &ImageDetail,
        compress: bool,
    ) -> Result<usize, usize> {
        let md5 = image.md5.clone();
        let tmp_string: Vec<&str> = image.file_url.split("/").collect();
        let ext_tmp: Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();

        let config = CONFIG.get().expect("error load config");
        let sys_config = &config.system.clone();
        // 使用 let 绑定来存储 String 克隆
        let path_string = sys_config.path.clone();
        let origin_string = sys_config.origin_img.clone();
        let webp_string = sys_config.webp_path.clone();

        let mut full_name = format!(
            "{}/{}/{}/{}.{}",
            path_string,
            origin_string,
            &md5[0..2],
            &md5,
            ext
        );
        if compress {
            full_name = format!(
                "{}/{}/{}/{}.{}",
                path_string,
                webp_string,
                &md5[0..2],
                &md5,
                "webp"
            );
        }
        let img_res = tokio::fs::read(&full_name).await;
        match &img_res {
            Ok(d) => {
                img_data.extend_from_slice(d.as_slice());
                return Ok(img_data.len());
            }
            Err(e) => {
                println!("read image error: {} {}", e, &full_name)
            }
        }
        return Err(0);
    }
    pub fn get_content_type(ext: &str) -> &'static str {
        return if ext == "png" {
            "image/png"
        } else if ext == "jpg" {
            "image/jpeg"
        } else if ext == "webp" {
            "image/webp"
        } else {
            "image/jpeg"
        }
    }
    pub fn get_file_ext(filename: &str) -> &str {
        let tmp_string: Vec<&str> = filename.split("/").collect();
        let ext_tmp: Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        return ext;
    }
    pub async fn sample_image_post(
        params: HashMap<String, String>,
    ) -> Result<Response<String>, Rejection> {
        let condition = SearchCondition::parse(params)?;

        let image = sample_one(condition)
            .await
            .unwrap_or_default();
        let resp = Response::builder()
            .header("content-type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&image).unwrap())
            .unwrap();
        Ok(resp)
    }
    pub async fn sample_image(
        params: HashMap<String, String>,
    ) -> Result<Response<Vec<u8>>, Rejection> {
        let condition = SearchCondition::parse(params)?;
        let image = sample_one(condition.clone())
            .await
            .unwrap();

        let compress = condition.clone().compress.unwrap();
        let ext = {
            if compress {
                "webp"
            } else {
                get_file_ext(image.file_url.as_str())
            }
        };

        let mut img_data = vec![];
        let start_time = SystemTime::now();
        if read_image(&mut img_data, &image, compress).await.is_err() {
            let resp = Response::builder()
                .header("content-type", "application/json")
                .body(vec![])
                .unwrap();
            return Ok(resp);
        }
        redis_incr_key("traffic", img_data.len()).await;
        let read_time = SystemTime::now().duration_since(start_time).unwrap();
        println!("read time: {:?}", read_time);
        let content_type = get_content_type(ext);
        let resp = Response::builder()
            .header("content-type", content_type)
            .header("image_id", image._id)
            .header("image_source", image.source)
            .header("image_tags", image.tags.join(","))
            .body(img_data)
            .unwrap();
        Ok(resp)
    }
    pub async fn sample_image_redirect(
        params: HashMap<String, String>,
    ) -> Result<impl warp::Reply, warp::Rejection> {

        let condition = SearchCondition::parse(params)?;
        let domain = {
            let config = CONFIG.get().expect("");
            let domain_index = 0;

            config.host.domain[domain_index].as_str()
        };

        let compress = condition.clone().compress.unwrap();

        let image = sample_one(condition)
            .await
            .unwrap();
        let ext = get_file_ext(image.file_url.as_str());

        let mut target_link: String = format!("https://{:}/{}/{}.{}", domain,&image.md5[0..2],&image.md5,ext);

        if compress {
            redis_incr_key("traffic", image.file_size / 3).await;
            target_link = format!("{}_webp",target_link);
        } else {
            redis_incr_key("traffic", image.file_size).await;
        }
        Ok(
            Response::builder()
                .status(302)
                .header("location",target_link)
                .body(hyper::Body::empty())
        )
    }
}
