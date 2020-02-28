use std::io;

use actix_files as fs;
use actix_web::{error,web,guard,Resource,Result,HttpResponse};
use actix_web::http::StatusCode;

/**
 * 处理 404 请求
 */
pub fn service_404() -> Resource {
    return web::resource("")
               .route(web::get().to(p404))
               // 所有不是`GET`的请求
               .route(web::route().guard(guard::Not(guard::Get())).to(HttpResponse::MethodNotAllowed))
}


/**
 * 404请求回调函数
 */
async fn p404() -> Result<fs::NamedFile> {
    
    return Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND));
}


/**
 * /error 请求地址处理
 */
pub fn service_error() -> Resource {
    return web::resource("/error").to(|| async {
        // 直接返回错误信息
        return error::InternalError::new(
            io::Error::new(io::ErrorKind::Other, "服务器发生错误"),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    });
}

