/**
 * String类型简单使用（注意：&str表示基础值，String 表示&str的包装对象，里面包含很多操作函数）
 */
fn main() {
    // 初始化一个String
    let mut s1 = String::from("测试String");
    // 拼接字符串
    s1.push_str("+++");

    let s2 = String::from("添加");
    // 使用引用拼接字符串
    s1.push_str(&s2);
    // 添加一个字符
    s1.push('d');
    s1.push('鼎');

    println!("s1={}",s1);
    println!("s1的长度={}",s1.len());
    println!("将s1转换成字节数组={:?}",s1.as_bytes());
    println!("将s1转换成字节数组={:?}",s1.bytes());
    println!("将s1转换成字符数组={:?}",s1.chars());

    let aa = vec![229,165,189];
    println!("利用UTF8编码将数组转换成String={}",String::from_utf8_lossy(&aa));

    let sss = format!("{}-{}",s1,2);
    println!("格式化完成的字符串 sss={}",sss);

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
    test_split();
}

/**
 * 分割字符串
 */
fn test_split(){
    let s = String::from("dadas sdfsfsd sdfsdfg fdgdf");
    //s.split(",");
    for item in s.split_whitespace() {
        println!("根据空格切分：{}",item);
    }
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
