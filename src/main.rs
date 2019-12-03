// 创建第三方库（这个是生成随机数的包）
extern crate rand;
// 引入标准的IO库
use std::io;
use rand::Rng;
use std::cmp::Ordering;
/**
   *  程序入口函数
 */
fn main() {
    // 生成5-100的随机数（注意：101是不会有的）
    // :: 的作用其实和java的new一个对象差不多（这个说法有误）
	let secret_number = rand::thread_rng().gen_range(5,101);
	// 注意：! 表示可以使用{}取值
	println!("随机数：{}",secret_number);
	// 定义局部变量（注意：mut 标识该变量的值是可变的）
	let mut guess = String::new();
	// 程序执行到这里会等待控制台输入一行数据（以回车换行）（注意：.expect("Failed to read line") 是指程序错误时输出）
	io::stdin().read_line(&mut guess).expect("Failed to read line");
	// 将String类型的guess值，转成32位整型的数字
	let guess: u32 = guess.trim().parse().expect("字符串转数字异常");
    println!("控制台输入了：{}",guess);
    // 比较（里面的都是回调函数）
    match guess.cmp(&secret_number) {
	    Ordering::Less => println!("小于"),
        Ordering::Greater => println!("大于"),
        Ordering::Equal => println!("相等"),
    }
}
