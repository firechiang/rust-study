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
#### 四、[IDEA安装Rust编辑插件][1]
#### 五、[IDEA创建Rust项目][2]
#### 六、[IDEA引入Rust项目][3]

[1]: https://github.com/firechiang/rust-study/tree/master/docs/idea-install.md
[2]: https://github.com/firechiang/rust-study/tree/master/docs/idea-create-project.md
[3]: https://github.com/firechiang/rust-study/tree/master/docs/idea-import-project.md
