fn main() {
    let s = String::from("hello world!");

    let len = s.len();

    let slice = &s[3..len];

    println!("slice value is:{}", slice);

    let slice = &s[..7];
    println!("slice value is:{}", slice);

    let slice = &s[..];
    println!("slice value is:{}", slice);
}
