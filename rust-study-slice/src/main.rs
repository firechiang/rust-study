/**
 * 字符串和数组截取简单使用
 */
fn main() {
    let mut s = String::from("sdfsd sdsfds");
    let indexOf = test_string(&s);
    // 截取字符串s 0-5的位置产生一个引用（注意：这个引用还是字符串s的引用，只不过它只指向0-5位置的字符）
    let a = &s[0..5];
    // 截取字符串s 6-10的位置产生一个新的引用（注意：这个引用还是字符串s的引用，只不过它只指向6-10位置的字符）
    let b = &s[6..10];
    // 从0开始截一直到最后
    let c = &s[..];
    // 从0开始截一直到len位置
    let len = s.len();
    let d = &s[0..len];
    println!("a={},b={},c={},d={}",a,b,c,d);
    // 清空变量s的值，让其变成""（注意：注释打开会报错，因为后面的代码又用到了s变量，所以不能将其清空）
    //s.clear();
    println!("空格在第{}个位置",indexOf);
    let aa = test_string1(&s[..]);
    println!("a={}",a);
    test_array();
}

/**
 * 截取字符串
 */
fn test_string1 (s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        // 当前字节的值是空格
        if item == b' ' {
            // 从s中截取0到i的位置，产生一个引用并返回（注意：这个引用还是字符串s的引用，只不过它只指向0-i位置的字符）
            return &s[0..i];
        }
    }
    // 从s中截取0到最后的位置，产生一个引用并返回（注意：这个引用还是字符串s的引用，只不过它只指向0-最后位置的字符）
    return &s[..];
}

/**
 * 截取数组
 */
fn test_array () {
    let array1 = ["1","sd","dsf","dfs","fd"];
    // 截取0到2的位置，并获取它的引用
    let array2 = &array1[0..2];
    // 截取全部
    // let array3 = &array1[..];
    // 从0截取到最后
    // let array4 = &array1[0..array1.len()];
    println!("array2={}",array2[0]);
}

fn test_string (s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        // 当前字节的值是空格
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}


