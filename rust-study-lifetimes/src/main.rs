/**
 * 标识生相同生命周期示例（将参数和返回值标识为具有相同的生命周期或对象和属性具有相同的生命周期）
 */
fn main() {

    let r;
    {
        let s = 5;
        r = &s;
    }
    // 放开注释报错，因为r是在另一个作用域里面赋的引用，而那个引用出了作用域就已经被销毁了，所以报错
    //println!("r={}",r);

    let novel = String::from("标识生命周期，简单使用");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // 注意：如果ImportantExcerpt类没有标识要和part具有相同的生命周期，下面的代码是会报错的（）
    let i = ImportantExcerpt { part: first_sentence};
    println!("i={:?}",i);
}
// 这个函数是编译不过的因为参数和返回值用的是同一个值，但是参数和返回值都有各自的生命周期，编译期间不知道以哪个生命周期为准，所以报错
/*fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/

/**
 * 'a 是生命周期标识（如果参数或返回值有 'a 就表示参数和返回值的生命周期相同）
 * 注意：'a这个符号可以随便起，只要是以'开头就可以
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/**
 * 表示生命周期的类（这个表示类对象和属性part具有相同的生命周期（因为part是个引用类型，它的生命周期很可能过期，加了生命周期标识，只要类的生命周期不过期，part引用就不过期））
 * 注意：类里面如果有属性使用了引用类型，那么一定要加上生命周期标识（也就是属性引用和类对象生命周期相同）
 * 注意：'a这个符号可以随便起，只要是以'开头就可以
 */
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}
