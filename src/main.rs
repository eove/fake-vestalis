#[macro_use]
extern crate rocket;
extern crate serde;

use base64::Engine;
use hmac::{Hmac, Mac};
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use sha2::Sha512;
use std::env;
use urlencoding::encode;
use uuid::Uuid;

type HmacSha512 = Hmac<Sha512>;

fn hmac_key() -> String {
    env::var("HMAC_KEY").expect("A key should be given for hmac computation") // TODO crash at startup
}

#[derive(Serialize)]
struct SignedData {
    token: String,
    uuid: String,
}

#[get("/sign/<target>/<timestamp>")]
fn hmac_sign(target: &str, timestamp: &str) -> Json<SignedData> {
    let uuid = Uuid::new_v4().to_string();
    let json = format!(
        r#"{{ "target":"{}", "timestamp":"{}", "uuid":"{}" }}"#,
        target, timestamp, uuid
    );
    let mut mac = HmacSha512::new_from_slice(hmac_key().as_bytes()).unwrap();
    mac.update(json.as_bytes());
    let hash = mac.finalize();
    let token = encode(&base64::prelude::BASE64_STANDARD.encode(hash.into_bytes())).to_string();
    Json(SignedData { token, uuid })
}

struct ConnectName(String);

#[get("/connect-name")]
fn connect_name(name: &State<ConnectName>) -> Json<String> {
    Json(name.0.clone())
}

#[launch]
fn rocket() -> _ {
    let name = env::var("CONNECT_NAME")
        .expect("CONNECT_NAME should be set to know which hostname to call");
    rocket::build()
        .manage(ConnectName(name))
        .mount("/", routes![hmac_sign, connect_name])
        .mount("/", FileServer::from("static/"))
}
