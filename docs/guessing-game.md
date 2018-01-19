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
use std::io

fn main() {
    println!("Guess the number!");
    
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .except("Failed to read line.");
        
    println!("Your guessed: {} ");
}
```