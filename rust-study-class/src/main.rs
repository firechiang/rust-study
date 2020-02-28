/**
 * Rust的类简单使用（注意：默认属性和函数都是私有的）
 */
fn main() {
    // 初始化一个类（而且是可变的）
    let mut user1 = User {
        username: String::from("毛毛"),
        email: String::from("a4545145646@qq.com"),
        age: 22,
        active: true
    };
    let name_age = test_user_age(&user1);
    user1.set_age(100);
    println!("user.age={}",user1.get_age());
    // 注意：一定要有:?否则无法打印
    println!("user{:?}",user1);
    // 初始化一个简单的类
    let dept = Dept(12,12,45);
    println!("dept.1={}",dept.1);
    println!("dept={:?}",dept);
    // 使用自己写的一个函数创建user对象
    let user2 = User::new(String::from("maomao"),String::from("a54548454@qq.com"),32,false);
    println_user(&user2);
    println!("user2={:?}",user2);
}

/**
 * 定义一个user类
 * 注意：加了#[derive(Debug)]就等于加了toString可以直接调用打印函数
 */
#[derive(Debug)]
struct User {
    // 用户名
    username: String,
    // 邮箱
    email: String,
    // 年龄
    age: u32,
    // 是否可用
    active: bool,
}
/**
 * 定义User类里面的函数（说明这个函数是User类里面的）
 * 注意：如果函数不需要当前类的实列，是可以直接使用::调用的（就像JAVA的静态函数一样）
 */
impl User {
    //获取age属性的值
    // 注意 &self 就是指当前对象的引用（Java的this想当），如果是self说明就是当前对象的值
    fn get_age (&self) -> u32 {

        return self.age;
    }
    // 设置age的值（设置值的话，当前对象的引用要是可变的）
    // 注意：在当前对象初始化的时候要加上mut说明其是可变的
    fn set_age (&mut self,age: u32) {

        self.age = age;
    }
    // 这个可以直接使用::调用的，因为它不需要当前类的实列
    fn new(username: String,email: String,age: u32,active: bool) -> User {
        return User {
            username: username,
            email: email,
            age: age,
            active: active
        }
    }
}

/**
 * 定义一个简单的类
 * 注意：加了#[derive(Debug)]就等于加了toString可以直接调用打印函数
 */
#[derive(Debug)]
struct Dept (
    i32,
    u32,
    i16
);

fn test_user_age (user: &User) -> String {
    let age = user.age;
    return String::from("sdsdfsd");
}

fn println_user(user: &User){
    println!("user={:?}",user);
}



