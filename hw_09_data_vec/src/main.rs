/**
 * 动态数组简单使用
 */
fn main() {
    // 创建一个动态数组
    let mut vec:Vec<u32> = Vec::new();
    // 添加数据
    vec.push(10);
    println!("vec={:#?}",vec);

    // 静态创建一个动态数组
    let mut vec1: Vec<i32> = vec![0,1,2,3,4,5,6,7];
    
    // 截取数组0到4的位置（注意：第4个位置不会取，实际是0到3的位置，也就是4个元素）
    let vec2: Vec<i32> = &vec1[..4];
    println("vec2: {:#?}",&vec2);

    // 截取数组1到4的位置（注意：第4个位置不会取，实际是1到3点位置，也就是3个元素）
    let vec3: Vec<i32> = &vec1[1..4];
    println("vec3: {:#?}",vec3);
    
    // 删除最后一个数据
    let index = vec1.pop();
    // 给数组扩容
    vec1.capacity();
    println!("vec1={:#?}",vec1);
    // 迭代动态数组
    for i in 0..vec1.len() {
        print!("index={}",vec1[i]);
    }
    println!("");
    // 迭代动态数组
    for e in vec1.iter() {
        print!("index={}",e);
    }
    // 迭代并修改每一个元素的值
    for e in vec1.iter_mut() {
        *e = *e * 2;
    }
    println!("vec1={:#?}",vec1);

}
