pub mod api{
    use crate::database::handler::handler::sample_one;
    use warp::{Filter, Rejection};
    use warp::http::Response;

    pub fn api_sample() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
        warp::any()
            .and(warp::path("image"))
            .and_then(sample_image)
    }
    pub async fn sample_image() -> Result<Response<Vec<u8>>, Rejection> {
        let image = sample_one().await.unwrap();
        let md5 = image.md5;
        let tmp_string:Vec<&str> = image.file_url.split("/").collect();
        let ext_tmp:Vec<&str> = tmp_string.last().unwrap().split(".").collect();
        let ext = ext_tmp.last().unwrap();
        let full_name = format!("/home/images_opt/{}/{}_optmized.{}",&md5[0..2],&md5,ext);
        println!("{}",full_name);
        let img_data = tokio::fs::read("C:\\Users\\Image\\Pictures\\微信图片_20201209135523.jpg").await.unwrap();
        let resp = Response::builder().header("content-type","image/jpeg")
            .body(img_data).unwrap();
        Ok(resp)
    }
}