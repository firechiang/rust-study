/**
 * 可变数组简单使用
 */
fn main() {
    // 初始化一个没有元素的可变数组（u32表示泛型）
    let mut v1: Vec<u32> = Vec::new();
    v1.push(1);
    v1.push(1);
    v1.push(3);
    // 删除尾部元素
    v1.pop();
    // 在第1个位置插入元素20
    v1.insert(1,20);
    // 这个获取到的是Option包装过的数据
    println!("获取到v1第0个元素={:?}",v1.get(0));
    // 这个获取到的是原始数据
    println!("获取到v1第1个元素={}",v1[1]);
    println!("v1={:?}",v1);

    // 根据元素初始化可变数组
    let mut v2 = vec![5,6,7,];
    v2.push(78);
    println!("v2={:?}",v2);
    test();

    // 循环修改可变数组里面的值
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v={:?}",v);

    // 可变数组里面装枚举类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row={:?}",row);
}

fn test () {
    let mut v = vec![1,4,6,7,8,9];
    let s = &v[0];
    println!("s={}",s);
    v.push(53);
    println!("v={:?}",v);
    // 注意注释放开就会报错，因为s变量的引用地址已经改了（原因是v.push(53)这行代码，给原始数组添加了数据，导致引用地址变了）
    //println!("s={}",s);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
