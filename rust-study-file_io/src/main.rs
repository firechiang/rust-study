use std::env;
use std::fs;
use std::process;
/**
 * 文件读取以及错误处理和命令行参数的获取
 * 使用命令执行该程序，在当前文件目录下，执行命令：cargo run somebody content.txt（注意：somebody和content.txt表示是参数）
 */
fn main() {
    // 获取main函数的所有参数
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        args.insert(1,String::from("somebody"));
        args.insert(2,String::from("content.txt"));
    }
    let (query,filename) = parse_config(&args);
    // |err| {} 是一个用于回调的闭包函数（就是JAVA里面的匿名函数）
    let config = Config::from(&args).unwrap_or_else(|err| {
        println!("解析参数时出现问题: {}", err);
        // 程序退出
        process::exit(1);
    });
    println!("在 {} 文件里面，查找\"{}\"字符串",config.filename,config.query);
    // 注意 &* 是将 String 强制转换成 str
    let errorMsg = &*format!("文件 {} 读取失败",config.filename);
    let content = fs::read_to_string(config.filename).expect(errorMsg);
    println!("文件里面的内容：{}",content);
    let con = search(&config.query,&content);
    println!("包含 {} 的内容：{:?}",query,con);

    // 获取环境变量
    //let val = env::var("环境变量的key");
    //env::var("环境变量的key").is_err();
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn from(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("没有3个参数");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn search<'a> (query: &str,content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in content.lines() {
        // to_lowercase()转换成小写，contains()是否包含
        if line.to_lowercase().contains(query) {
            res.push(line);
        }
    }
    return res;
}


fn parse_config (args: &[String]) -> (&str,&str) {
    let query = &args[1];
    let filename = &args[2];
    return (query,filename);
}
