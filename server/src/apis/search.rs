use actix_web::{get, HttpResponse, web};
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, options::ClientOptions, Client, bson::oid::ObjectId};
use crate::common::response::{PaginationInfo, RSearchHttpJsonResponse};
// traits needed for cursor
use futures::stream::{StreamExt};


#[derive(Clone, Debug, Deserialize, Serialize)]
struct Movie {
    _id: ObjectId,
    release_year: u32,
    title: String,
    origin: Option<String>,
    director: Vec<String>,
    cast: Vec<String>,
    genre: Vec<String>,
    wiki_page: String,
    plot: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchQuery {
    p: u32,
    ps: u32,
    q: String,
}

#[get("/search")]
pub async fn search(info: web::Query<SearchQuery>) -> HttpResponse {
    let mut client_options = ClientOptions::parse("mongodb://rsearch:password@database:27017").await.unwrap();
    client_options.app_name = Some("RSearch".to_string());
    let client = Client::with_options(client_options).unwrap();

    let mut data = Vec::<Movie>::new();
    let matched_count = client
        .database("rsearch")
        .collection::<Movie>("movie")
        .find(doc! {"$text": {"$search":info.q.clone()}}, None)
        .await.unwrap().count().await;
    let mut cursor = client
        .database("rsearch")
        .collection::<Movie>("movie")
        .find(doc! {"$text": {"$search": info.q.clone()}}, None)
        .await.unwrap();
    let mut index = 0;
    while let Some(m) = cursor.next().await {
        if index >= info.p * info.ps {
            break;
        }
        if index >= (info.p - 1) * info.ps && index < info.p * info.ps {
            data.push(m.unwrap());
        }
        index += 1;
    }
    let page_info = PaginationInfo {
        page_index: info.p,
        page_size: info.ps,
        page_total: (matched_count as u32 / info.ps) as u32,
    };
    RSearchHttpJsonResponse::new(page_info, data)
}