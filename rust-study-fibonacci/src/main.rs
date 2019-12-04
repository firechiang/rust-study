fn main() {
    println!("请输入要计算的数据：");
    let mut strr = String::new();
    std::io::stdin().read_line(&mut strr).expect("命令行输入异常");
    let strr: u32 = match strr.trim().parse() {
        // 如果输入的是数字就直接返回
        Ok(num) => num,
        // 如果出现错误（注意：err回调函数里面一定要退出程序，否则编译无法通过）
        Err(e) => {
            println!("计算的数据有误，程序正在退出：{}",e);
            sleep_1();
            // 退出程序
            std::process::exit(0);
        }
    };
    println!("计算结果： {}",fibonacci(strr));
    sleep_1();
}


/**
 * 程序暂停3秒
 */
fn sleep_1(){
    std::thread::sleep(std::time::Duration::from_secs(3));
}

/**
 * 斐波那契数
 */
fn fibonacci(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}