use serde::{Deserialize, Serialize};

use actix_session::Session;
use actix_web::{post, web, HttpRequest, HttpResponse, Result};

#[derive(Deserialize)]
struct Identity {
    user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[post("/login")]
async fn login(user_id: web::Json<Identity>, session: Session) -> Result<HttpResponse> {
    let id = user_id.into_inner().user_id;

    let username = format!("User{}", id);
    let login_user = User { id, username };

    session.insert("user_id", &id)?;
    session.insert("user", &login_user)?;
    session.renew();

    //let counter: i32 = session.get::<i32>("conter").unwrap_or(Some(0)).unwrap_or(0);
    //log::info!("counter: {}", counter);
    // let id = 1; //todo
    Ok(HttpResponse::Ok().json(login_user))
}

#[post("/logout")]
async fn logout(session: Session) -> Result<String> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(x) = id {
        session.purge();
        Ok(format!("Logged out: {}", x))
    } else {
        Ok("Could not log out anonymous user".into())
    }
}
