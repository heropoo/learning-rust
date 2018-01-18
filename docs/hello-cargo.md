## 使用Cargo创建项目

### 创建项目
```bash
cd rust-projects
cargo new hello_cargo --bin
```

说明
* 使用`--bin`参数，说明我们的目标是生成一个可执行程序，而不是一个库。
* Cargo 生成了一个文件和一个目录：一个`Cargo.toml`文件(项目的配置文件)和一个`src`目录（代码源文件目录），`main.rs`文件位于`src`目录中。
* 同时初始化了一个git仓库以及一个`.gitignore`文件, 你可以通过`--vcs`参数切换到其它版本控制系统（VCS），或者不使用 VCS。


### 构建运行
```
cargo build
```
输出
```
   Compiling hello_cargo v0.1.0 (file:///path/to/rust-projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.55 secs
```
首次运行`cargo build`的时候，Cargo 会在项目根目录创建一个锁文件`Cargo.lock`

运行
```
./target/debug/hello_cargo
```
输出
```
Hello, world!
```

也可以使用`cargo run`同时编译并运行
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

### 发布（release）构建
当项目最终准备好发布了，可以使用`cargo build --release`来优化编译项目。这会在`target/release`而不是`target/debug`下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种为了开发，你需要经常快速重新构建；另一种为了构建给用户最终程序，它们不会重新构建，并且希望程序运行得越快越好。如果你在测试代码的运行时间，请确保运行`cargo build --release`并使用`target/release`下的可执行文件进行测试。