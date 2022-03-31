/**
 * 错误处理简单实用
 */
fn main() {
    let a = 10;
    if a == 11 {
        // 一般代码遇到某种错误程序要退出时使用
        panic!("程序退出!，后面的代码不会执行");
    }
    // 断言错误处理
    assert();
    let res2:Result<u32,&'static str> = Result::Ok(1);
    let res3:Result<u32,&'static str> = Result::Err("出现了错误");
    println!("res2={:#?}",res2);
    println!("res3={:#?}",res3);

    let r = std::fs::read("src/main.rs");
    // 使用match 加 Result 进行错误处理
    match r {
        Ok(data) => {
            // 注意：使用unwrap()函数，如果前面的代码异常，调用该函数程序将会退出
            let d = std::str::from_utf8(&data).unwrap();
            println!("{}",d)
        }
        Err(err) => {
            println!("程序出错")
        }
    }

    read_file_data();
}

/**
 * 错误传递简单使用
 */
fn foo() -> Result<i32,&'static str>{
    // ？表示如果程序出错误，错误会自动向上传递
    let x = bar()?;
    Ok(x as i32)
}

fn bar() -> Result<u32,&'static str> {
    Ok(0)
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::ErrorKind),
    Utf8Error(std::str::Utf8Error)
}

// 实现错误自动转换（std::io::Error错误自动转换成我们自定义的Error）
impl core::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error.kind())
    }
}

// 实现错误自动转换（std::str::Utf8Error错误自动转换成我们自定义的Error）
impl core::convert::From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Utf8Error(error)
    }
}

// 注意：Result<(),Error> 括号代表无值并不是任意值
fn read_file_data() -> Result<(),Error> {
    // ？表示表示有错误时自动向上传递，也就是自动返回Error（注意：本来这个错误类型是std::io::Error，但是我们需要自己定义的Error，而且我们也实现了转换，所以它会自动转换的）
    let data = std::fs::read("src/main.rs")?;
    // ？表示表示有错误时自动向上传递，也就是自动返回Error（注意：本来这个错误类型是std::str::utf8Error，但是我们需要自己定义的Error，而且我们也实现了转换，所以它会自动转换的）
    let data_str = std::str::from_utf8(&data)?;
    println!("main_data={}",data_str);
    Ok(())
}


fn add(a:u32,b:u32) -> u32 {
    // 未实现的代码，类似于Python的pass关键字
    unimplemented!()
}

fn divide_by_three(x: u32) -> u32 {
    for i in 0.. {
        if 3 * i < i {
            panic!("程序退出!");
        } else {
            return i - 1;
        }
    }
    // 代码不会走到这里来，但是程序又必须返回，可以使用该宏，使代码保证完整性
    unreachable!()
}

fn assert() {
    let a = 10;
    let b = 10;
    assert!(a == b);
}
