// use blog::run;
mod application;

use application::run;

fn main() {
    let host = "127.0.0.1:7878";

    println!("Listening on http://{}", host);

    run(&host);
}
