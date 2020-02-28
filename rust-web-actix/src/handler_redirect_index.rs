use actix_web::{web,HttpRequest,HttpResponse,Resource};
use actix_web::http::header;

/**
 * 请求首页重定向到static/welcome.html页面
 */
pub fn service_redirect_index() -> Resource {

    return web::resource("/").route(web::get().to(|req: HttpRequest| {
        println!("请求首页信息 {:?}", req);
        // 在返回的头信息里面添加要重定向的地址（注意：客户端获取到这个地址以后会发起一个新的请求）
        return HttpResponse::Found().header(header::LOCATION, "static/welcome.html").finish();
    }));
}