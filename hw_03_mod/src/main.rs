// 导入模块并起了个别名
use std::fs as my_fs;
// 引入当前目录下的某个代码文件
mod test_mod;

/**
 * 创建一个模
 */
mod mod1 {
    // 默认是私有的，只有在当前mod1模块内才能使用
    const NAME:&str = "maomoa";

    // 加了self表示在当前模块内和当前模块的子模块内可见
    pub(self) enum CrateEnum {
        Nmae
    }

    // 加了crate表示当前项目内可见，如果当前项目被当成第三方依赖，那么在第三方项目里面是不可见的
    pub(crate) enum Alphat {
        A,
        B,
    }
    /**
     * 在模块里面添加子模块
     */
    pub mod mod2 {
        fn test() {
            // 使用super关键字访问上层模块里面的内容
            let name = super::NAME;
        }
    }
}

fn main() {
    // 读取文件里面的所有内容
    let data = std::fs::read("src/main.rs").unwrap();
    println!("main.rs文件里面的内容={}",String::from_utf8(data).unwrap());
    test_mod::test_test();
}
