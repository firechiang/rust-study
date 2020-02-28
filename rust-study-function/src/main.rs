use std::borrow::Borrow;

/**
 * 函数的声明和使用以及代码块的简单使用
 */
fn main() {
    test_on_param(0,1);
    test_code_block();
    println!("函数返回了：{}",test_method_return());
    println!("函数返回了：{}",test_param_method_return(16));
}

/**
 * 带参数并且带返回值的函数简单使用
 */
fn test_param_method_return(x: u32) -> u32 {
    // 注意： return关键字可以不写（注意：如果不写return的话，分号也不要写，直接写个 x + 10 就可以了）
    return x + 10;
}

/**
 * 带返回值的函数简单使用
 */
fn test_method_return () -> String {
    // 注意： return关键字可以不写（注意：如果不写return的话，分号也不要写，直接写个String::from("a")就可以了）
    return String::from("a");
}

/**
 * 带参数的函数（u32和i16表示参数的类型）
 */
fn test_on_param(x: u32,y: i16) {
    println!("test_on_param函数执行了，参数x={},y={}",x,y);
}

/**
 * 测试代码块赋值的简单使用
 */
fn test_code_block() {
    let x = 5;
    // y等于t+1
    let y = {
        let t = 10;
        // 表示返回t + 1
        t + 1//（注意：这理不能写分号，否则报错。因为这个一行表示返回。可以写成 return t + 1;）
    };
    println!("x={},y={}",x,y);
}