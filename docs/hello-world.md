## 第一个rust程序-HelloWorld

### 创建工作目录
```bash
mkdir rust-projects
midir -p rust-projects/hello_world
cd rust-projects/hello_world
```

### 编写代码文件`main.rs`
```rust
fn main() {
    println!("Hello world!");
}
```

### 编译
```bash
rustc main.rs
```
如果代码没有错误，编译后生成 main 的可执行文件

### 运行
```bash
./main
```
输出
```
Hello world!
```

### 备注
* `println!()` 这称为[Rust宏](https://doc.rust-lang.org/book/first-edition/macros.html#macros)
