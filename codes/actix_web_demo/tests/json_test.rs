use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    username: String,
    password: String,
    created_at: u64,
    updated_at: u64,
    phones: Vec<String>
}

#[test]
fn json_encode() {
    let user = User {
        id: 1,
        username: String::from("小明"),
        password: String::from("123456"),
        created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        phones: vec!["13812341234".to_string(), String::from("13812341235")]
    };

    // Serialize it to a JSON string.
    let user_string = serde_json::to_string(&user).unwrap();

    println!("{}", user_string);
}

#[test]
fn json_decode(){
    let user_string = r#"
    {
        "id":1,
        "username":"小明",
        "password":"123456",
        "created_at":12312,
        "updated_at":1232134343
    }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(user_string).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["username"], v["created_at"]);
    assert_eq!(1, 1)
}

#[test]
fn json_decode2(){
    let user_string = r#"
    {
        "id":1,
        "username":"小明",
        "password":"123456",
        "created_at":12312,
        "updated_at":1232134343,
        "phones":["13812341234","13812341235"]
    }"#;

    // Parse the string of data into serde_json::Value.
    let user: User = serde_json::from_str(user_string).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", user.username, user.phones[0]);
    assert_eq!(1, 1)
}
