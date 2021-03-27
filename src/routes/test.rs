use actix_web::{web, HttpResponse};

use std::time::Duration;

use serde::{Deserialize, Serialize};

use derive_more::{Display, Error};

const LONG_DURATION: Duration = Duration::from_secs(2);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestResponseData {
    some_data: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    some_query_param: String,
}

pub async fn test_route(info: web::Query<Info>) -> HttpResponse {
    let data = TestResponseData {
        some_data: String::from(&info.some_query_param),
    };

    tracing::info!("test");
    tracing::warn!("some req");

    tracing::warn_span!("some data", data = 42);
    // tracing::event!(Level::WARN, answer = "foo", question = "life, the universe, and everything");


    HttpResponse::Ok().json(data)
}
