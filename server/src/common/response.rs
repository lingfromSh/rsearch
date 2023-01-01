use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, http::header::{ContentType}};

#[derive(Serialize, Deserialize)]
pub struct PaginationInfo {
    // 当前页码
    pub page_index: u32,
    pub page_total: u32,
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
        HttpResponse::Ok().content_type(ContentType::json()).json(body)
    }
}