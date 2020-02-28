use actix_web::http::Method;
use actix_web::{web,HttpRequest,HttpResponse,Resource};

/**
 * /test 地址请求的回调函数
 */
pub fn test() -> Resource {
    // 匹配请求方式
    return web::resource("/test").to(|req: HttpRequest | match *req.method() {
        // GET 请求直接返回OK
        Method::GET => HttpResponse::Ok(),
        // POST 直接返回失败
        Method::POST => HttpResponse::MethodNotAllowed(),
        // 默认返回没有请求
        _ => HttpResponse::NotFound()
    });
}