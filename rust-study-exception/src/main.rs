use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
/**
 * 异常处理简单使用（注意：结尾的?号表示如果处理失败，直接返回Error）
 */
fn main() {
    //panic!("程序发生错误，并退出");
    //test_error();
    // 打开一个文件
    test_file_open();
    // 默认找classpath目录下的文件
    let f1 = File::open("gui-config.json").expect("文件打开异常");
    let f2 = File::open("gui-config.json").unwrap();
    println!("获取到文件f1：{:?}",f1);
    println!("获取到文件f12：{:?}",f2);
    let res = test_file_read1();
    println!("读取到了文件里面的内容，打印枚举类型：{:?}",res);

    let content = std::fs::read_to_string("gui-config.json");
    println!("直接读取到了文件里面的内容：{:?}",content.unwrap());
    let sss = read_username_from_file();
    println!("简写的方式读取到了文件里面的内容：{}",sss.unwrap());
}


/**
 * 正常读取文件里面的内容,使用自定义的枚举返回（注意：String表示OK的泛型，io::Error表示Err的泛型）
 */
fn test_file_read1 () -> Res<String,io::Error> {
    let file = File::open("gui-config.json");
    let mut file = match file {
        Ok(file) => file,
        // 如果文件读取异常，程序直接返回，match外面的下面的代码也不会执行
        Err(e) => return Res::Err(e)
    };
    println!("文件读取成功，打印日志");
    let mut s = String::new();
    return match file.read_to_string(&mut s) {
        // 内容读取成功将字符串 s 装入 枚举类型返回
        Ok(_) => Res::Ok(s),
        Err(e) => Res::Err(e)
    };
}

/**
 * 使用简写的方式读取文件里面的内容（注意：String表示OK的泛型，io::Error表示Error的泛型）
 */
fn read_username_from_file() -> Result<String, io::Error> {
    // 打开文件（?号表示如果打开失败，直接返回Error）
    let mut f = File::open("gui-config.json")?;
    let mut s = String::new();
    // 读取文件内容（?号表示如果读取失败，直接返回Error）
    f.read_to_string(&mut s)?;
    // 读取成功，返回成功
    Ok(s)
}


fn test_file_open () {
    let file = File::open("gui-config.json");
    //let file = File::open("D:\\shadowsocks-4.1.6\\gui-config.json").expect("打印的错误信息");
    //println!("file={:?}",file.unwrap());
    // 匹配是否发生异常
    let file = match file {
        // 正常返回File对象
        Ok(file) => file,
        // 匹配错误类型
        Err(error1)  => match error1.kind() {
            // 如果是文件不存在的错误（就创建文件）
            ErrorKind::NotFound => match File::create("gui-config.json"){
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

#[derive(Debug)]
enum Res<T, E> {
    Ok(T),
    Err(E),
}



