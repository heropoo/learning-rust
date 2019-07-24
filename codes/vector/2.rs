fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(thir) => println!("The third element is {}", thir),
        None => println!("There is no third element."),
    }
}
