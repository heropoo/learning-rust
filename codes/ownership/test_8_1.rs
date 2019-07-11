fn main() {
    let a = dangle();
    println!("a value is: {}", a);
}

fn dangle() -> String {
    let s = String::from("hello"); // s 是一个新字符串

    s // 返回字符串 s 
}

// 引用的规则
// - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// - 引用必须总是有效。
