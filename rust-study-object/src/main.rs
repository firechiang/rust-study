/**
 * 面向对象的封装
 */
fn main() {
    let a = AveragedCollection {
        list: vec![1,2,3],
        average: 1 as f64,
        name: String::from("ssdsdfds")
    };
    let b = Button {
        width: 8,
        height: 8,
        label: String::from("sadas")
    };
    test_draw(a);
    test_draw(b);
}

/**
 * 创建一个类
 */
#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
    name: String,
}
/**
 * AveragedCollection类里面的函数
 */
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    /**
     * 设置name的值
     */
    pub fn set_name (&mut self,name: String) {
        self.name = name;
    }
    /**
     * 获取name的值
     */
    pub fn get_name (&self) -> &String {
        return &self.name;
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// 定义一个接口
pub trait Draw {
    fn draw(&self);
}

// AveragedCollection实现Draw接口
impl Draw for AveragedCollection {

    fn draw(&self) {
        println!("调用了 {:?} 的draw函数",self);
    }
}
// 定义一个类
#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Button实现Draw接口
impl Draw for Button {
    fn draw(&self) {
        println!("调用了 {:?} 的draw函数",self);
    }
}

//use std::any::*;

/**
 * 测试传入Draw接口的参数
 */
fn test_draw (d: impl Draw) {

    d.draw();
}

