#### 一、Linux 开发环境搭建，[官方安装文档](https://www.rust-lang.org/learn/get-started)
```bash
# 命令执行执行完成以后会有一些提示让配置好环境变量，一般默认安装在用户目录下.cargo目录
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# rust 更新（注意：这个rustup命令是安装好rust环境以后才会有）
$ rustup update

# rust安装其它指定版本
$ rustup install 1.66.1

# rust使用指定版本
$ rustup default 1.66.1

# 显示rust已安装的所有版本
$ rustup show
```
#### 二、Rust配置国内软件源，[官方国内配置源文档](http://mirrors.ustc.edu.cn/help/crates.io-index.html)
```bash
# 编辑 $HOME/.cargo/config 文件添加下面的内容（注意：$HOME/.cargo/ 是Rust环境安装目录）
$ vi $HOME/.cargo/config

[source.crates-io]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 测试验证国内源
$ cargo search
```
#### 三、[IDEA安装Rust编辑插件][1]
#### 四、[IDEA创建Rust项目][2]
#### 五、[IDEA引入Rust项目][3]
#### 六、[IDEA引入Rust模块][4]
#### 七、Rust 编成规范
 - 直接使用引用传递效率更高
 - return 关键字，表示程序不会再向下执行会立即返回（包括在match里面写return，match外面的下面的代码也不会执行） 
 - 在代码返回的位置，最好使用return关键字并加上分号（如果不写return关键字，那么也不要写分号（因为分号表示代码不返回））
 
[1]: https://github.com/firechiang/rust-study/tree/master/docs/idea-install.md
[2]: https://github.com/firechiang/rust-study/tree/master/docs/idea-create-project.md
[3]: https://github.com/firechiang/rust-study/tree/master/docs/idea-import-project.md
[4]: https://github.com/firechiang/rust-study/tree/master/docs/idea-import-module.md
