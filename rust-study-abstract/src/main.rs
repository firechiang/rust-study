/**
 * 接口定义和实现简单使用
 */
fn main() {
    let tweet = Tweet {
        username: String::from("毛毛"),
        content: String::from("描述"),
        reply: false,
        retweet: false,
    };
    println!("创建一个 Tweet: {}", tweet.summarize());
    let t = returns_summarizable();
}

/**
 * 定义一个接口
 */
trait Summary {
    /**
     * 接口里面的一个函数
     */
    fn summarize(&self) -> String;
    /**
     * 这个相当于接口里面的默认实现函数
     */
    fn def_summarize(&self) -> String {
        // 调用抽象函数（注意：这个实际调用的是实现类的函数）
        let s = self.summarize();
        return format!("(Read more from {}...)", s);
    }
    /**
     * 定义一个接口函数（参数t要实现Display和Clone接口，参数u要实现Debug和Clone接口）
     */
    fn some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(t: T, u: U) -> i32;
    /**
     * 接口函数的定义和上面的那个函数一样，只是写法不一样（使用where关键字来表示泛型T的实际类型）
     */
    fn some_function1<T, U>(t: T, u: U) -> i32 where T: std::fmt::Display + Clone,U: Clone + std::fmt::Debug;
}

/**
 * 定义一个接口
 */
trait Summary1 {

}

/**
 * 创建一个类（注意：这个类在下面实现了多个接口）
 */
#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

/**
 * 使用Tweet类来现Summary1接口
 */
impl Summary1 for Tweet {

}

/**
 * 使用Tweet类来现Summary接口
 */
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    /**
 * 定义一个接口函数（参数t要实现Display和Clone接口，参数u要实现Debug和Clone接口）
 */
    fn some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(t: T, u: U) -> i32 {

        return 1;
    }
    /**
     * 接口函数的定义和上面的那个函数一样，只是写法不一样
     */
    fn some_function1<T, U>(t: T, u: U) -> i32 where T: std::fmt::Display + Clone,U: Clone + std::fmt::Debug {

        return 2;
    }
}

/**
 * 该函数的返回值是实现了 Summary 接口的实列
 */
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("提阿尼"),
        content: String::from("我是实现了 Summary 接口的实列"),
        reply: false,
        retweet: false,
    }
}

