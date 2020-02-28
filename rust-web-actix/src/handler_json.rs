use actix_web::{Error,HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use futures::future::{ready, Ready};

use bytes::Bytes;
use futures::stream::once;
use futures::future::ok;

#[derive(Serialize)]
struct MyObj {
    name: String,
}


impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

/**
 * 直接返回JSON
 */
pub async fn index() -> impl Responder {
    MyObj { name: String::from("name") }
}

/**
 * 流式响应
 */
pub async fn index2() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}