use std::net::IpAddr;
/**
 * 类型强制转换简单使用
 */
fn main() {
    let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
    println!("转换的IP={}",ip);
    let s = String::from("32");
    let i = s.parse::<u32>().unwrap();
    println!("转换成功的数字i={}",i);
    let s1 = i.to_string();
    println!("转换后的string={}",s1);
    // 将一个字符串转换成IpAddr对象
    let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
    println!("ip={}",ip);
    String_to_str();
    i32_to_u32();
}

fn i32_to_u32() {
    let i: i32 = 51;
    // i32转u32
    let u: u32 = i as u32;
    println!("i32转u32的结果u32={}",u);
    let f = u as f64;
    println!("u32转f64的结果f64={}",f);
}

fn String_to_str() {
    let string:String = String::from("hello");
    let strs:&str = &*string;
    println!("String转str的结果strs={}",strs);
}
