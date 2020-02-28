/**
 * 基础数组和tuple高级数组的使用方法
 */
fn main() {
    test_simple_array();
    test_senior_array();
}

/**
 * 数组简单使用（注意：简单数组里面的元素类型必须一致）
 */
fn test_simple_array() {
    // 数组简单定义
    let array1 = [1, 2, 3, 4, 5];
    println!("获取数组元素：{}", array1[0]);
    // [u32; 1] 表示数组元素的类型是u32，1表示只有1个元素
    let array2: [u32; 1] = [1];
    println!("获取数组元素：{}", array2[0]);
    // 初始化一个数组里面的元素是5个3
    let array3 = [3; 5];
    println!("第5个元素={}", array3[4]);
    println!("数组的长度是{}",array3.len());
}

/**
 * 高级数组简单测试（注意：高级数组里面的元素类型可以不一样）
 */
fn test_senior_array() {
    // 定义一个高级数组（里面放的是固定3种类型的数据，后面是赋值）
    let tup: (u32, i16, &str) = (32, -15, "1");
    // 定义一个不知道类型的高级数组
    let tup1 = (32, -15, 1);
    // 定义一个固定的数组
    let (x, y, z) = tup;
    println!("x={},y={},z={}", x, y, z);
    println!("数组1元素={},数组2元素={},数组3元素={}", tup.0, tup.1, tup.2);
    println!("获取数组的第一个元素={}", tup.0);
    println!("获取数组的最后一个元素={}", tup1.2);
}
