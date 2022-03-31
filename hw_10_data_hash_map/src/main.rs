use std::collections::HashMap;
/**
 * HashMap简单使用
 */
fn main() {
    let mut map: HashMap<&str,u32> = HashMap::new();
    // 插入数据
    map.insert("maomao",90);
    // 获取值
    let inn = match map.get(&"maomao") {
        Some(data) => data,
        None => &0
    };
    println!("map={:#?}",map);
    // 迭代HashMap（注意：&name前面加&是因为我们迭代的时候获取到的引用，前面加&表示将引用转化成值）
    for(&name,value) in map.iter() {
        println!("name={},value={}",name,value);
    }
}
