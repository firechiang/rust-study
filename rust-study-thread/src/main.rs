use std::thread;
use std::time::Duration;
/**
 * 线程相关
 */
fn main() {
    simulated_expensive_calculation(2);
    let intensity = 10;
    // 定义一个闭包函数变量（就是这个变量是一个函数，num是函数的参数）
    let expensive_closure = |num| {
        println!("正在调用闭包函数...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 使用闭包函数变量
    expensive_closure(intensity);

}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("任务开始执行...");
    // 线程等待2秒
    thread::sleep(Duration::from_secs(2));
    intensity
}
