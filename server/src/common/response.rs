use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, http::header::{ContentType}};

#[derive(Serialize, Deserialize)]
pub struct PaginationInfo {
    // current page index
    pub page_index: u32,
    // total count of pages
    pub page_total: u32,
    // size of each page
    pub page_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RSearchHttpJsonResponse<D: Serialize>
{
    page_info: PaginationInfo,
    data: D,
}

impl<D: Serialize> RSearchHttpJsonResponse<D> {
    pub fn new(page_info: PaginationInfo, data: D) -> HttpResponse {
        let body = Self {
            page_info,
            data,
        };
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .json(body)
    }
}