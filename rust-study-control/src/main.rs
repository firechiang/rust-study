use std::time::Duration;

/**
 * 循环语句for,while,loop用法以及使用loop循环的返回值进行赋值简单使用
 */
fn main() {
    test_if();
    test_if_assignment();
    test_loop();
    test_loop_val();
    test_while();
    test_for_1();
    test_for_array();
    test_for_array1();
}

/**
 * for循环简单使用
 */
fn test_for_1(){
    // 正序循环
    for item in (1..4) {
        println!("正序循环,元素{}",item);
    }
    // 倒叙后再循环
    for item in (1..4).rev() {
        println!("倒叙后再循环,元素{}",item);
    }
}

/**
 * for循环数组简单使用（有打印下标）
 */
fn test_for_array() {
    let array = ["1","2","sdsa","sdfsd"];
    for (i,item) in array.iter().enumerate() {
        println!("循环数组元素下标{}={}",i,item);
    }
}

/**
 * for循环数组简单使用
 */
fn test_for_array1() {
    let array = ["1","2","sdsa","sdfsd"];
    for item in array.iter() {
        println!("循环数组元素{}",item);
    }
}

/**
 * while循环简单使用
 */
fn test_while(){
    let mut x = 1;
    while x != 10 {
        x += 1;
    }
    println!("while循环结束");
}

/**
 * 死循环简单使用
 */
fn test_loop() {
    let mut x = 1;
    loop {
        x+=1;
        println!("x={}",x);
        // 暂停1秒（注意 ::极有可能表示的是静态函数调用）
        std::thread::sleep(Duration::from_secs(1));
        // 当x等于3的时候退出循环
        if x == 3 {
            println!("退出死训话");
            break;
        }
    }
}

/**
 * 死循环返回值简单使用
 */
fn test_loop_val () {
    let mut x = 1;
    // 将循环体里面的返回值赋给y
    let y = loop {
        x+=1;
        if x == 10 {
            // 跳出循环并返回x的值
            break x + 1;
        }
    };
    println!("y的值={}",y);
}

/**
 * 将if代码块里面的返回值，赋给其它变量简单使用
 */
pub fn test_if_assignment(){
    let is = true;
    let x = 10;
    // 将if代码块里面的返回值赋给y
    let y = if is {
        // 返回1（注意：不能写分号）
        1
    }else{
        // 返回2（注意：不能写分号）
        2
    };
    println!("y={}",y);
}

/**
 * if语句简单使用
 */
pub fn test_if(){
    let x = 10;
    let y = "10";
    if x == y.len() {
        println!("相等");
    }else{
        println!("不相等");
    }
}
