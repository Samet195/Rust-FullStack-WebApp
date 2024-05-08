use common::types::{ErrorJson, TestData};

use actix_web::*;
use std::{fs, path::PathBuf};

#[get("/test")]
pub async fn test_d() -> HttpResponse {
    test_impl(None).await
}

#[get("/test/{dir:.*}")]
pub async fn test(dir: Option<web::Path<String>>) -> HttpResponse {
    test_impl(dir).await
}

async fn test_impl(dir: Option<web::Path<String>>) -> HttpResponse {
    let mut path = PathBuf::new();
    path.push(".");

    #[cfg(target_os = "android")]
    path.push("/data/data/com.islam.bookshelf");

    if let Some(dir) = dir {
        path.push(PathBuf::from(dir.as_str()));
    }

    match fs::read_dir(path) {
        Ok(o) => {
            let data: TestData = o.into();
            HttpResponse::Ok().json(data)
        }
        Err(e) => HttpResponse::NotFound().json(ErrorJson {
            message: format!("Error: {}", e),
        }),
    }
}
