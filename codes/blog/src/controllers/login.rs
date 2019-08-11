pub fn login() -> (&'static str, &'static str) {
    ("HTTP/1.1 200 OK\r\n\r\n", "tpls/login.html")
}

pub fn logout() -> String {
    String::from("login - logout")
}
