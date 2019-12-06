use std::fs::File;
use std::error::Error;
use std::io::*;
use std::io;
use std::fs;

/**
 * 异常处理简单使用
 */
fn main() {
    //panic!("程序发生错误，并退出");
    //test_error();
    // 打开一个文件
    test_file_open();
}

fn test_file_open () {
    let file = File::open("D:\\shadowsocks-4.1.6\\gui-config.json");
    //let file = File::open("D:\\shadowsocks-4.1.6\\gui-config.json").expect("打印的错误信息");
    //println!("file={:?}",file.unwrap());
    // 匹配是否发生异常
    let file = match file {
        // 正常返回File对象
        Ok(file) => file,
        // 匹配错误类型
        Err(error1)  => match error1.kind() {
            // 如果是文件不存在的错误（就创建文件）
            ErrorKind::NotFound => match File::create("D:\\shadowsocks-4.1.6\\gui-config.json"){
                // 文件创建成功
                Ok(file) => file,
                // 文件创建异常
                Err(e) => {
                    panic!("文件创建失败，程序退出，错误信息{}",e);
                }
            },
            // 默认错误处理
            other_error=> {
                panic!("程序发生未知错误，程序退出，错误信息{:?}",other_error);
            }
        }
    };
    println!("file={:?}",file);
    println!("打开文件里面的内容：{:?}",file.metadata());
}

fn test_error () {
    let array = [1,2,3];
    println!("测试程序发生错误，并退出{}",array[1000]);
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
