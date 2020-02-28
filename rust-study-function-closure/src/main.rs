use std::thread;
use std::time::Duration;
/**
 * 闭包函数简单使用（就是JAVA的匿名函数和Lambda表达式）
 */
fn main() {
    let intensity = 10;
    // 定义一个没有明确返回值类型的闭包函数变量（就是这个变量是一个函数，num是函数的参数）
    let expensive_closure = |num| {
        println!("正在调用闭包函数...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 使用闭包函数变量
    expensive_closure(intensity);

    // 有明确返回值类型的闭包函数
    let expensive_closure1 = |num: u32| -> u32 {
        println!("正在调用闭包函数...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 使用闭包函数变量
    expensive_closure1(intensity);

    // 以下全是定义闭包函数的方法
    fn  add_one_v1   (x: u32) -> u32 { x + 1 };
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32|             { x + 1 };
    let add_one_v4 = |x: u32|               x + 1  ;

    generate_workout(1,2);
}

/**
 * 泛型是一个闭包函数，并闭包函数的返回值类型是u32类型
 * 注意：Fn 表示只读不写, FnMut 能读能写, FnOnce 只传一次
 */
struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // value有值，就返回
            Some(v) => v,
            // value没有值
            None => {
                // 调用自身的闭包函数，取值
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

/**
 * 测试Cacher类的函数
 */
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("开始执行第一个任务...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
