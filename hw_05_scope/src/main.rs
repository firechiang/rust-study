/**
 * 作用域简单使用
 */
fn main() {
    let a = 10;
    {
        let b = 20;
        println!("b={}",b);
    }
    println!("a={}",a);

    let name = String::from("maomoa");
    let name1 = name;
    //println!("name={}",name); 因为所有权的转移，所以name已经被销毁了不能用了
    println!("name1={}",name1);

    let name2 = String::from("tiantian");
    // 引用传递（Rust专业叫法叫借用）,值将不会被修改
    test_scope2(&name2);
    test_scope(name2);
    //println!("name2={}",name2); name2已经被使用过了，这里将无法使用

    let mut str_ref = "cccc";
    let str_ref1 = &mut str_ref;
    let str_ref2 = &mut str_ref;
    // println!("str_ref1={},str_ref2={}",str_ref1,str_ref2); 一个可变引用不能同时拥有多个可变使用
    println!("str_ref1");
}

// 'a是用来标注生命周期的，在这里是标识name属性的的生命周期和结构体的生命周期一致，都是'a嘛
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
}

// 'a是用来标注生命周期的，在这里是标识返回值的生命周期和参数的生命周期一致，都是'a嘛
fn bigger<'a>(str1: &'a str,str2: &'a str)-> &'a str {
    if str1 > str2 {
        str1
    } else {
        str2
    }

}

fn test_scope(str: String) {
    println!("str={}",str);
}

// 引用传递（Rust专业叫法叫借用）
fn test_scope2(str: &String) {
    println!("str={}",str);
}
