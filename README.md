# learning-rust
Learning rust-lang

## 国内更换源
今天学cargo第一节，`cargo build`时安装外部依赖真的慢啊。果断找国内源,果断又是ustc(中科大, emmm以前应该努力一点考中科大~(￣▽￣)~\*)

教程开始：

设置两个环境变量
```bash
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```
或者直接写入`~/.bashrc`

使用ustcu的源下载安装rust
```
curl -sSf https://mirrors.ustc.edu.cn/rust-static/rustup.sh | sh
```
编辑`~/.cargo/config`写入
```
[registry]
index = "https://mirrors.ustc.edu.cn/crates.io-index/"
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "https://mirrors.ustc.edu.cn/crates.io-index/"
```
现在cargo安装外部依赖是不是贼溜~

## 目录：
* [安装](./docs/install.md)
* [第一个rust程序-HelloWorld](./docs/hello-world.md)
* [使用Cargo创建项目](./docs/hello-cargo.md)
* [猜猜看小游戏](./docs/guessing-game.md)

参考：
* 官方的：[《The Rust Programming Language》](https://doc.rust-lang.org/book/)
* 中文翻译版：[《Rust 程序设计语言（第一版）》](https://www.gitbook.com/book/kaisery/rust-book-chinese)
* 中文翻译版：[《Rust 程序设计语言（第二版）》](https://kaisery.github.io/trpl-zh-cn)


