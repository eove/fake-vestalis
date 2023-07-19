#[macro_use]
extern crate rocket;
extern crate serde;

use aes_gcm::aead::Aead;
use aes_gcm::aead::OsRng;
use aes_gcm::KeyInit;
use aes_gcm::{AeadCore, Aes256Gcm};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{Signer, SigningKey};
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::{env, fs};
use urlencoding::encode;
use uuid::Uuid;

fn read_private_key() -> SigningKey {
    let path =
        env::var("SIGNATURE_KEY").expect("SIGNATURE_KEY must be given for signing computation");
    let public_key_bytes = fs::read_to_string(path).unwrap();
    SigningKey::from_pkcs8_pem(&public_key_bytes).unwrap()
}

#[derive(Serialize)]
struct SignedData {
    signature: String,
    data: String,
    nonce: String,
}

#[derive(Serialize, Debug)]
pub struct EncodedData<'r> {
    target: &'r str,
    timestamp: &'r str,
    uuid: &'r str,
}

fn encode_string_data(data: &str) -> (String, String) {
    let key = env::var("CIPHER_KEY").unwrap();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let aes = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = aes.encrypt(&nonce, data.as_bytes()).unwrap();
    (
        BASE64_STANDARD.encode(ciphertext),
        BASE64_STANDARD.encode(nonce),
    )
}

#[get("/sign/<target>/<timestamp>")]
fn ed25519_sign(target: &str, timestamp: &str) -> Json<SignedData> {
    let uuid = Uuid::new_v4().to_string();
    let data_to_encode = EncodedData {
        target,
        timestamp,
        uuid: uuid.as_str(),
    };
    let json = serde_json::to_string(&data_to_encode).unwrap();
    let (encrypted_json, nonce) = encode_string_data(&json);
    let key = read_private_key();
    let signed = key.sign(&encrypted_json.as_bytes());
    let signature = encode(&BASE64_STANDARD.encode(signed.to_bytes())).to_string();
    let data = encode(&encrypted_json).to_string();
    let nonce = encode(&nonce).to_string();
    Json(SignedData {
        signature,
        data,
        nonce,
    })
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
        .mount("/", routes![ed25519_sign, connect_name])
        .mount("/", FileServer::from("static/"))
}
