#[macro_use]
extern crate actix_web;
use actix_utils::mpsc;
use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_web::{web,middleware,Error,App,HttpRequest,HttpServer,HttpResponse/*,Responder*/,Result};
use actix_web::http::StatusCode;
use std::env;
use bytes::Bytes;
// 引入当前目录的模块(注意：这个模块里面还包含有其它模块)
mod handler_error;
mod handler_redirect_index;
mod handler_test;
mod handler_json;
mod handler_urlencoded;

/**
 * 测试使用actix-web框架开发Web应用
 * #[actix_rt::main] 宏表示该函数是异步的（另起一个线程）
 */
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 设置日志级别
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // 初始化日志
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // 开启并使用Cookie
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // 开启记录请求日志
            .wrap(middleware::Logger::default())
            // 添加自定义头信息
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            // 开启压缩
            .wrap(middleware::Compress::default())
            .service(favicon)
            .service(index)
            .route("/hello/{name}", web::get().to(hello))
            .route("/json", web::get().to(handler_json::index))
            .route("/json2", web::get().to(handler_json::index2))
            .route("param",web::get().to(handler_urlencoded::get_params))
            .route("param1",web::get().to(handler_urlencoded::get_params1))
            .route("param2",web::get().to(handler_urlencoded::get_params2))
            .route("param3",web::post().to(handler_urlencoded::get_params2))
            // 异步返回 response body
            .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
            // 暴露静态目录（暴露后里面的文件可以直接请求的到，如果不暴露，里面的文件是请求不到的）
            .service(fs::Files::new("/static", "static").show_files_listing())
            // test 请求处理（里面有匹配请求方式）
            .service(handler_test::test())
            // 重定向简单使用
            .service(handler_redirect_index::service_redirect_index())
            // 请求直接返回500
            .service(handler_error::service_error())
            // 404 处理
            .default_service(handler_error::service_404())
    }).bind("127.0.0.1:8000")?.run().await
}


/**
 * 获取服务器的图标（这个可做为文件下载）
 */
#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

/**
 * 返回HTML信息
 */
#[get("/index")]
async fn index(session: Session/*, req: HttpRequest*/) -> Result<HttpResponse> {
    //println!("{:?}", req);
    // 获取session的值
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("获取到 SESSION value: {}", count);
        counter = count + 1;
    }
    // 设置 session
    session.set("counter", counter)?;
    return Ok(HttpResponse::build(StatusCode::OK).content_type("text/html; charset=utf-8").body(include_str!("../static/welcome.html")))
}

/**
 * 返回自定义HTML信息
 */
async fn hello(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {
    println!("请求 /hello的信息 {:?}", req);
    // 获取请求地址上面name的值，没获取到就是使用`默认值`（注意：这个值可以直接用 path.0 获取）
    //let name = req.match_info().get("name").unwrap_or("默认值");
    HttpResponse::Ok()
                 .content_type("text/plain; charset=utf-8")
                 .body(format!("Hello {}!", path.0))
}

/**
 * 返回 response body
 */
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}


