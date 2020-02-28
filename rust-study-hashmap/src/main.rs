/**
 * HashMap简单使用
 */
fn main() {
    // 引入HashMap
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("a",89);
    map.insert("b",45);
    println!("map={:?}",map);
    // 注意这个返回的是Option包装过的对象
    println!("获取HashMap的值，a={:?}",map.get("a"));
    println!("获取HashMap的一个键值对entry={:?}",map.entry("a"));
    // 如果map里面没有kkk，就插入kkk它的值是56（注意：这个执行完成以后会返回插入value的引用）
    map.entry("kkk").or_insert(56);
    println!("map={:?}",map);

    // 将两个数组合并成一个HashMap
    let vec = vec!["sadas","sda","4545"];
    let nums = vec![324,33,344];
    let groupMap: HashMap<_,_> = vec.iter().zip(nums.iter()).collect();
    println!("并后的HahsMao，groupMap={:?}",groupMap);


    let text = "hello world wonderful world";
    // 根据单词切割
    let ws = text.split_whitespace();
    println!("根据空格切割后的结果ws={:?}",ws);
    let mut map1 = HashMap::new();
    for word in text.split_whitespace() {
        println!("word={}",word);
        // 返回插入value的引用
        let count = map1.entry(word).or_insert(0);
        // 修改value引用的值
        *count += 1;
    }
    println!("{:?}", map1);
}
