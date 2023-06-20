#[macro_use]
extern crate rocket;
extern crate serde;

use base64::Engine;
use hmac::{Hmac, Mac};
use rocket::fs::FileServer;
use sha2::Sha512;
use std::env;
use urlencoding::encode;

type HmacSha512 = Hmac<Sha512>;

fn hmac_key() -> String {
    env::var("HMAC_KEY").expect("A key should be given for hmac computation") // TODO crash at startup
}

#[get("/sign/<target>/<timestamp>")]
fn hmac_sign(target: &str, timestamp: &str) -> String {
    let json = format!(
        r#"{{ "target":"{}", "timestamp":"{}" }}"#,
        target, timestamp
    );
    let mut mac = HmacSha512::new_from_slice(hmac_key().as_bytes()).unwrap();
    mac.update(json.as_bytes());
    let hash = mac.finalize();
    encode(&base64::prelude::BASE64_STANDARD.encode(hash.into_bytes())).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hmac_sign])
        .mount("/", FileServer::from("static/"))
}
