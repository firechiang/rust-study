/**
 * 基础数据类型说明和使用（Signed=有符号，Unsigned=无符号）
 * |---------|---------|-----------|
 * | Length	 | Signed  | Unsigned  |
 * |---------|---------|-----------|
 * | 8-bit	 |  i8	   |  u8       |
 * |---------|---------|-----------|
 * | 32-bit	 |  i32	   |  u32      |
 * |---------|---------|-----------|
 * | 64-bit	 |  i64	   |  u64      |
 * |---------|---------|-----------|
 * | 128-bit |  i128   |  u128     |
 * |---------|---------|-----------|
 * | arch	 |  isize  |  usize    |
 * |---------|---------|-----------|
 *
 * Signed表示这个变量是有符号的，可以存储 整数 和负数
 * Unsigned表示这个变量是没有符号的，只能存储正数
 * 注意： Signed存储符号是有代价的，代价就是存储空间中的一个比特位，专门用来存储符号，这一位不能表示数值
 *       所以一般来说同类型的Signed能够存储的数值的绝对值大小要小于Undigned
 *
 * @sine（Signed和Unsigned存储说明） https://www.cnblogs.com/yema/p/5202949.html
 */
fn main() {
    // 定义常量 AAA_B（:u32表示数字类型，是32位的正整形）
    const AAA_B: u32 = 32000;
    // 定义常量 AAA_C（i16表示数字类型，是16位的整形，正负数都可以）
    const AAA_C: i16 = 31000;
    println!("AAA_B长量: {}", AAA_B);
    println!("AAA_C长量: {}", AAA_C);
    test_data_type();
    test_float();
    test_decimal();
    test_digital_operation();
    test_boolean();
}

/**
 * 进制表示
 */
fn test_decimal(){
    // 十进制的98222
    let x = 98_222;
    // 0x开头表示十六进制
    let y = 0xff;
    // 0o开头表示八进制
    let u = 0o77;
    // 0b开头表示二进制
    let c = 0b1111_0000;
    // b开头是字节型的表示是8位无符号的
    let b = b'A';
    print!("x={}",x);
    print!("y={}",y);
    print!("u={}",u);
    print!("c={}",c);
    print!("b={}",b);
    println!("");
}


/**
 * Boolean类型的数值定义
 */
fn test_boolean(){
    let a = true;
    // : bool表示这个是Boolean类型
    let b: bool = false;
    println!("{},{}",a,b);
}

/**
 * 数字运算
 */
fn test_digital_operation(){
    // 加
    let sum = 5 + 10;
    // 减
    let difference = 95.5 - 4.3;
    // 乘
    let product = 4 * 30;

    // 除
    let quotient = 56.7 / 32.2;
    // 求余
    let remainder = 43 % 5;

    println!("{},{},{},{},{}",sum,difference,product,quotient,remainder)
}

fn test_float(){
    // 默认是64位浮点类型（也就是8个字节）
    let x = 2.1;
    // 指定是32位浮点类型（也就是4个字节）
    let y: f32 = 20.0;
    println!("两个Float的和: {}",x+y);
}

fn test_data_type() {
    let str = "40";
    let str: u32 = str.parse().expect("转换成数字类型失败");
    println!("转换之后的值：{}",str);
}


