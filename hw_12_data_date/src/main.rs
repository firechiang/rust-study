use std::process::exit;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
/**
 * 时间处理简单使用
 */
fn main() {
    // 获取当前系统时间
    let now = SystemTime::now();
    println!("now={:#?}",now);
    // 获取 timestamp (注意：SystemTime::UNIX_EPOCH 表示timestamp起始时间，也就是1970年1月1日到现在的秒数)
    // 注意：如果操作系统将时间调到1970年以前就会报错
    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH);
    println!("timestamp={:#?}",timestamp);
    // 程序暂停4秒
    sleep(Duration::from_secs(3));
    // 获取并打印程序休眠时间
    println!("ela={:#?}",now.elapsed().unwrap());
    // 获取60秒以后的时间
    let future = now.checked_add(Duration::from_secs(60));
    println!("future={:#?}",future);
}
