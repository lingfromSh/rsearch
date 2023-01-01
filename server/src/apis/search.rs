use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::common::response::{PaginationInfo, RSearchHttpJsonResponse};

#[derive(Deserialize, Serialize)]
struct WikiLink {
    title: String,
    link: String,
    rank: f64,
}

#[get("/search/")]
pub async fn search() -> HttpResponse {
    println!();
    let page_info = PaginationInfo {
        page_index: 0,
        page_total: 0,
        page_size: 0,
    };
    let data = WikiLink {
        title: "Title".to_string(),
        link: "link".to_string(),
        rank: 284.1_f64,
    };
    RSearchHttpJsonResponse::new(page_info, data)
}