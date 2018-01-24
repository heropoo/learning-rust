## 猜猜看小游戏

一个经典的新手编程问题：猜猜看游戏。程序将会随机生成一个 1 到 100 之间的随机整数。接着它会请玩家猜一个数并输入，然后提示猜测是大了还是小了。如果猜对了，它会打印祝贺信息并退出。

创建项目
```bash
cd rust-projects
cargo new guessing_game --bin
cd guessing_game
```

编辑`src/main.rs`文件
```rust
use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");
        
    println!("Your guessed: {} ", guess);
}
```
### 程序分析
```rust
use std::io;
```
`use`用于引入`io`(输入/输出)库引入到当前作用域。`io`库来自于标准库（也被称为`std`）

```rust
let mut guess = String::new();
```
* `let`用来创建 变量, 在Rust中，**变量默认是不可变的**。在变量名前使用`mut`(immutable/mutable)来使一个变量可变
* `String`是一个标准库提供的字符串类型，它是UTF-8编码的可增长文本块。`::new`那一行的`::`语法表明`new`是`String`类型的一个**关联函数**（associated function）。关联函数是针对类型实现的，在这个例子中是`String`，而不是`String`的某个特定实例。

```rust

```

* `&`表示这个参数是一个**引用**（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。它像变量一样，默认是不可变的，需要写成`&mut guess`而不是`&guess`来使其可变。
* 