use actix_web::{web,HttpResponse};
use std::collections::HashMap;
use serde::Deserialize;

/**
 * 接收 content-type 为 urlencoded 的参数（注意：如果不定义对象，就用HashMap接收）
 */
pub async fn get_params(name: web::Query<HashMap<String,String>>) -> HttpResponse {
    println!("获取到参数name={:?}",name);
    return HttpResponse::Ok().content_type("text/plain; charset=utf-8").body(format!("name={:?}",name));
}

/**
 * 接收 content-type 为 urlencoded 的参数（注意：如果直接使用对象接收，参数没传会报错）
 */
pub async fn get_params1(user: web::Query<User>) -> HttpResponse {
    println!("获取到参数name={:?}",user);
    return HttpResponse::Ok().content_type("text/plain; charset=utf-8").body(format!("name={:?}",user));
}

/**
 * 接收 content-type 为 urlencoded 的参数（注意：使用Option接收属性，如果参数没有传不会报错）
 */
pub async fn get_params2(user: web::Query<User1>) -> HttpResponse {
    println!("获取到参数name={:?}",user);
    return HttpResponse::Ok().content_type("text/plain; charset=utf-8").body(format!("name={:?}",user));
}

/**
 * 定义一个user类用来接收参数
 * 注意：#[derive(Debug)]就等于加了toString可以直接调用打印函数,#[derive(Deserialize)] 是解码参数
 */
#[derive(Debug,Deserialize)]
pub struct User {
    // 直接接收参数，如果参数没有传会报错
    name: String
}

/**
 * 定义一个user1类用来接收参数
 * 注意：#[derive(Debug)]就等于加了toString可以直接调用打印函数,#[derive(Deserialize)] 是解码参数
 */
#[derive(Debug,Deserialize)]
pub struct User1 {
    // 使用Option接收，如果参数没有传是不会报错的
    name: Option<String>
}