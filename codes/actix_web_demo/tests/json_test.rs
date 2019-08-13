use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    password: String,
    created_at: u32,
    updated_at: u32,
}

#[test]
fn json_encode() {
    let user = User {
        id: 1,
        username: String::from("小明"),
        password: String::from("123456"),
        created_at: 12312,
        updated_at: 1232134343,
    };

    // Serialize it to a JSON string.
    let user_string = serde_json::to_string(&user).unwrap();

    println!("{}", user_string);

    assert_eq!(1, 1)
}
