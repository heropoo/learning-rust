fn main() {
    let mut v = vec![2, 342, 235, 11, 21, -1];
    for i in &mut v {
        *i += 1;
        println!("i {}", i);
    }
}
