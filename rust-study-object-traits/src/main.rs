/**
 * 多态和重载以及重写的简单使用以及重写输出接口的格式化函数示例
 */
fn main() {
    let mut c = Counter{};
    let v = c.next();
    println!("v={:?}",v);
    // 重载函数的使用（注意：这个其实是实现了多个接口）
    let person = Human;
    // 调用Pilot接口的fly
    Pilot::fly(&person);
    // 这个写法和上面的写法等同(<类 as 接口>::函数())
    <Human as Pilot>::fly(&person);
    // 调用Wizard接口的fly
    Wizard::fly(&person);
    // 调用对象自身的fly
    person.fly();

    // 运算符的重载
    let m1 = Millimeters(10);
    let m2 = Meters(20);
    let m3 = m1 + m2;
    println!("m3={:?}",m3);

    // 方法重写只能在接口和接口上使用
    let pp = Point{x: 12,y:5};
    // 这个println输出被重写了
    println!("pp={}",pp);
    pp.outline_print();


    let w = Wrapper(vec![String::from("hello"), String::from("world"),String::from("tiantian")]);
    println!("w = {}", w);
}

// 定义一个接口
pub trait Iterator {
    // 这属性是个类型（实现了这个接口就要定义这个类型）
    type Item;
    // 获取下一个类型
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter{}

// 实现Iterator接口
impl Iterator for Counter {
    // 规定iter的类型是u32
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {

        return Some(121 as u32);
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // 重载函数
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add for Millimeters {
    type Output = Millimeters;
    // 重载函数
    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

use std::ops::Add;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

// 定义类型（用于使用重载函数）
struct Human;
struct Human2;

impl Pilot for Human {
    fn fly(&self) {
        println!("我实现的是Pilot接口");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("我实现的是Wizard接口");
    }
}

impl Human {
    fn fly(&self) {
        println!("就是Human里面的函数");
    }
}


use std::fmt;

// 定义接口OutlinePrint并实现fmt::Display
trait OutlinePrint: fmt::Display {
    // 重写outline_print函数
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// Point类实现接口OutlinePrint
impl OutlinePrint for Point {}

// 实现输出接口，并重写格式化函数
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait A {
    fn a(){
        println!("a");
    }
}
// 定义接口B并实现接口A
trait B : A {
    // 重写函数a
    fn a() {
        println!("b");
    }
}

struct Wrapper(Vec<String>);

// 实现输出接口，并重写格式化函数
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

