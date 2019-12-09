use std::net::IpAddr;
/**
 * 打印错误程序退出简单使用
 */
fn main() {
    // 将一个字符串转换成IpAddr对象
    let ip = "127.0.0.1".parse::<IpAddr>().unwrap();
    println!("ip={}",ip);
    let num: u32 = "221".parse::<u32>().unwrap();
    // 转换
    let num1: u32 = match "232323".parse::<u32>() {
        Ok(u) => u,
        Err(e) => {
            panic!("转换异常，程序退出")
        }
    };
    println!("转换成功，num1={}",num1);
}

struct Guess {
    num: u32
}

impl Guess {
    /**
     * 初始化
     */
    fn new (value: u32) -> Guess {
        if value > 1 || value < 100 {
            println!("value在100以内");
        }
        return Guess {
            num: value
        };
    }
    /**
     * get函数
     */
    fn getNum(&self) -> u32 {

        return self.num;
    }
}

