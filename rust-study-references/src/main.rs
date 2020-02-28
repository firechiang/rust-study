/**
 * 非基础数据类型引用传递和创建作用域简单使用（注意：同一作用域下一个变量可以有多个可读的引用，不能有多个可写的引用）
 */
fn main() {
    let s = String::from("参数的值");
    // 前面加&，表示传递的是引用
    test_references(&s);
    // 因为上面传递的是引用所以变量s在这里还是可用的
    println!("测试变量s是否还可用 s={}",s);
    let mut s1 = String::from("可以改变的测试");
    // 修改s1的值（加&mut表示可修改引用的值）
    test_references_mut(&mut s1);
    println!("修改后s1={}",s1);
    test_references_value();
    test_references_valu();
}

/**
 * 接收参数的引用（注意：参数类型前面加&表示接收的使用该类型的引用）
 * 注意：这样传递的引用，它的值是不可以改变的
 */
fn test_references (s: &String) {
    println!("接受到的参数 s ={}",s)
}

/**
 * 引用传递，并且可以改变引用的值（加&mut表示可修改引用的值）
 */
fn test_references_mut (s: &mut String) {
   s.push_str("新加了xxxx");
}

/**
 * 同一个作用域下，一个变量不能将可变引用同时赋给多个其它变量
 */
fn test_references_value () {
    let mut s = String::from("测试引用赋值");
    // 作用域一（中括号括起来的为一个作用域（这就是创建作用域））
    {
        let s2 = &mut s;
        println!("s2={}",s2);
    }
    // 作用域二
    let s1 = &mut s;
    //let s2 = &mut s; 注意：注释打开会报错，因为同一个作用域下，一个变量不能将可变引用同时赋给多个其它变量
    println!("s1={}",s1);
}

/**
 * 不可以变的借用，可以同时赋给多个变量，但是多个变量当中不能有可变赋予
 */
fn test_references_valu () {
    let mut s = String::from("sdfsdfsd");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; 注意：注释放开会报错，因为同一个作用域下不能有可变和不可变同时赋予
    println!("s1={},s2={}",s1,s2);
    // 注意：这个是可以的，因为println过后s1和s2已经没有再被使用（注意：如果后面不能再使用s1和s2，否则还是会报错）
    let s3 = &mut s;
    println!("s3={}",s3);
}

// 注意：这个函数是有问题的，因为变量s在函数调用完成以后就没有了，所以它是引用也是不可用的
/*fn dangle() -> &String {
    let s = String::from("hello");
    &s
}*/
