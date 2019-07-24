use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    let team_name = String::from("Blue");
    println!("Blue score is {:?}", scores.get(&team_name));

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
