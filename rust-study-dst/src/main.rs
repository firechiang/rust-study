/**
 * 不知道参数或变量具体占用内存大小的函数的写法包括泛型函数
 */
fn main() {
    println!("Hello, world!");
}

fn generic1<T>(t: T) {
    // --snip--
}


fn generic2<T: Sized>(t: T) {
    // --snip--
}

// 不知道参数的长度，在前面加个?号即可
fn generic3<T: ?Sized>(t: &T) {
    // --snip--
}
