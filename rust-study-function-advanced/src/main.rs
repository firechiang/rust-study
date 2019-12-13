/**
 * 参数是函数指针的函数简单使用（就是函数里面有一个参数是传一个函数的引用，在这个函数里面使用引用调用另一个函数）
 */
fn main() {
    // 带有函数指针的函数调用
    let v = do_witice(add_one,3);
    println!("v={}",v);

    let mut vs = vec![1,3,4,5];
    let vss: Vec<String> = vs.iter().map(ToString::to_string).collect();
    println!("vss={:?}",vss);

    // 这个的返回值是一个函数
    let ff = returns_closure();
    let ffvv = ff(55);
    println!("ffvv={}",ffvv);
}

// 一个普通函数
fn add_one (num: i32) -> i32 {

    num + 1
}

/**
 * f    函数指针参数返回值是i32（就是应该传一个函数给它，并且函数的返回值是i32类型）
 * args 普通的i32类型参数
 */
fn do_witice (f: fn(i32) -> i32,args: i32) -> i32 {
    // 调用两次传给它的f函数
    f(args) + f(args)
}

// 函数返回另一个函数指针,如果不用Box指针包起来将不能使用（注意：这个可以实现动态函数）
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {

    Box::new(|x| x + 1)
}

