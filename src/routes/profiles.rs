use crate::auth::Auth;
use crate::db;
use crate::models::user::Profile;
use rocket::serde::json::{json, Value};

fn to_profile_json(profile: Profile) -> Value {
    json!({ "profile": profile })
}

#[get("/profiles/<username>")]
pub fn get_profile(username: String, auth: Option<Auth>, conn: db::Conn) -> Option<Value> {
    let user_id = auth.map(|auth| auth.id);
    db::profiles::find(&conn, &username, user_id).map(to_profile_json)
}

#[post("/profiles/<username>/follow")]
pub fn follow(username: String, auth: Auth, conn: db::Conn) -> Option<Value> {
    db::profiles::follow(&conn, &username, auth.id).map(to_profile_json)
}

#[delete("/profiles/<username>/follow")]
pub fn unfollow(username: String, auth: Auth, conn: db::Conn) -> Option<Value> {
    db::profiles::unfollow(&conn, &username, auth.id).map(to_profile_json)
}
