use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut wirter = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut wirter).unwrap();
}
