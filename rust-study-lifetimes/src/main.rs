/**
 * 生命周期简单使用（注意：这一章得再看看）
 */
fn main() {

    let novel = String::from("标识生命周期，简单使用");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("i={:?}",i);
}
// 这个函数是编译不过的因为参数都是 &str，而且返回值也是 &str，所以它不知道具体使用那个生命周期
/*fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/

/**
 *  'a 是生命周期标识（如果参数或返回值有 'a 就表示生命周期的长度都相同）
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/**
 * 表示生命周期的类
 */
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}
