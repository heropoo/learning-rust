use std::fmt; // 导入 `fmt`

/// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
#[derive(Debug)]
struct Complex {
    real: f64, 
    imag: f64, 
}

/// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Complex {
    /// 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    // （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
    let s = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", s);
    println!("Debug: {:?}", s);
}
