#### Windows环境搭建，[参考博客](https://blog.csdn.net/weixin_43882409/article/details/87616268)
##### 一、配置CARGO_HOME和RUSTUP_HOME环境变量（注意：只需要配置两个目录即可，到时候Rust安装程序会自动将依赖安装到该目录）
#### 二、[下载安装Rust工具包](https://win.rustup.rs)
```bash
Current installation options:

# 以下是默认安装的组件
default host triple:  x86_64-pc-windows-msvc
default toolchain:    stable
profile:              default
modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
# 直接填1，安装默认组件
>1
```
#### 三、[下载安装Windows C++编译工具包，安装时建议只安装Windows10 SDK](http://go.microsoft.com/fwlink/?LinkId=691126)
#### 四、Linux 开发环境搭建，[官方安装文档](https://www.rust-lang.org/learn/get-started)
```bash
# 命令执行执行完成以后会有一些提示让配置好环境变量，一般默认安装在用户目录下.cargo目录
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### 五、Rust配置国内软件源，[官方国内配置源文档](http://mirrors.ustc.edu.cn/help/crates.io-index.html)
```bash
# 编辑 $HOME/.cargo/config 文件添加下面的内容（注意：$HOME/.cargo/ 是Rust环境安装目录）
$ vi $HOME/.cargo/config

[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index"

# 测试验证国内源
$ cargo search
```
#### 六、[IDEA安装Rust编辑插件][1]
#### 七、[IDEA创建Rust项目][2]
#### 八、[IDEA引入Rust项目][3]
#### 九、[IDEA引入Rust模块][4]
#### 十、Rust 编成规范
 - 直接使用引用传递效率更高
 - return 关键字，表示程序不会再向下执行会立即返回（包括在match里面写return，match外面的下面的代码也不会执行） 
 - 在代码返回的位置，最好使用return关键字并加上分号（如果不写return关键字，那么也不要写分号（因为分号表示代码不返回））
 
[1]: https://github.com/firechiang/rust-study/tree/master/docs/idea-install.md
[2]: https://github.com/firechiang/rust-study/tree/master/docs/idea-create-project.md
[3]: https://github.com/firechiang/rust-study/tree/master/docs/idea-import-project.md
[4]: https://github.com/firechiang/rust-study/tree/master/docs/idea-import-module.md
