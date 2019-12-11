/**
 * 单元测试类（注意：单元测试类的文件名一定要叫lib.rs，而且模块类上需要加上 #[cfg(test)] 注解才能表示测试类）
 * 注意：测试类的执行方法是在当前文件目录下，执行 cargo test 命令开始跑测试
 */
#[cfg(test)]
mod tests {

    /**
     * 测试函数（注意：需要加上 #[test] 注解才能表示测试函数）
     * 测试类执行方法：在当前文件目录下执行 cargo test 命令启动测试（注意：要加上 -- --nocapture 参数，才会执行打印语句）
     */
    #[test]
    fn test1 () {
        println!("正在执行单元测试类");
        // 断言等于函数
        assert_eq!(2 + 2, 4);
    }

    #[test]
    // 加了 #[should_panic(expected = "断言失败")] 注解表示断言失败才是正常的，断言不失败就不正常了
    #[should_panic(expected = "断言失败")]
    fn test2 () {
        //panic!("测试失败");
        assert!(false,"断言失败了value={}",false);
    }

    #[test]
    // #[ignore] 注解表示该函数不参与测试
    #[ignore]
    fn test3 () {
        assert!(false,"测试断言失败");
    }

    // 执行 cargo test add 命令执行测试（注意：加上add参数后，表示执行该命令后，只测试以add开头的函数）
    #[test]
    fn add_test () {
        assert!(true);
    }

    // 注意：如果没有这个，larger_can_hold_smaller函数里面的Rectangle将找不到
    use self::super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        // 断言函数
        assert!(larger.can_hold(&smaller));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}