/**
 * 变量重新赋值以及mut关键字修饰可变变量和使用const关键字修饰长量简单使用
 */
fn main() {
    const MU_TEST: u32 = 8;
    test_mut();
    test_shadowing();
}

/**
 * 測試shadowing就是使用let給同一块内存从新赋值
 */
fn test_shadowing() {
    // 定义String类型x
    let x = "10";
    // 使用let将string类型的x改变成int类型的值
    let x = x.len();
    println!("字符串x的长度：{}", x);
}

/**
 * 测试 mut 关键字，可变变量
 */
fn test_mut() {
    // 注意如果没有加 mut 关键字 x 是不能被重新赋值的（注意: mut关键字会强制数据类型）
    let mut x = 60;
    println!("我是旧值: {}", x);
    // 注意如果没有加 mut 关键字 x 是不能被重新赋值的（就相当于JAVA中的final变量）
    x = 71;
    println!("我是新值: {}", x);
}
