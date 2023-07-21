#[macro_use]
extern crate rocket;
extern crate serde;

use aes_gcm::aead::consts::U12;
use aes_gcm::aead::OsRng;
use aes_gcm::aead::{Aead, Nonce};
use aes_gcm::aes::Aes256;
use aes_gcm::{AeadCore, Aes256Gcm};
use aes_gcm::{AesGcm, KeyInit};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{Signer, SigningKey};
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::{env, fs};
use time::OffsetDateTime;
use urlencoding::encode;
use uuid::Uuid;

fn read_private_key() -> SigningKey {
    let path =
        env::var("SIGNATURE_KEY").expect("SIGNATURE_KEY must be given for signing computation");
    let public_key_bytes = fs::read_to_string(path).unwrap();
    SigningKey::from_pkcs8_pem(&public_key_bytes).unwrap()
}

#[derive(Serialize, PartialEq, Debug)]
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

fn encode_string_data(data: &str, nonce: &Nonce<AesGcm<Aes256, U12>>) -> String {
    let key = env::var("CIPHER_KEY").unwrap();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key.as_bytes());
    let aes = Aes256Gcm::new(&key);
    let ciphertext = aes.encrypt(nonce, data.as_bytes()).unwrap();
    BASE64_STANDARD.encode(ciphertext)
}

fn sign(
    data_to_encode: EncodedData,
    key: SigningKey,
    nonce: Nonce<AesGcm<Aes256, U12>>,
) -> SignedData {
    let json = serde_json::to_string(&data_to_encode).unwrap();
    let encrypted_json = encode_string_data(&json, &nonce);
    let signed = key.sign(&encrypted_json.as_bytes());
    let signature = encode(&BASE64_STANDARD.encode(signed.to_bytes())).to_string();
    let data = encode(&encrypted_json).to_string();
    let nonce = encode(&BASE64_STANDARD.encode(nonce)).to_string();
    SignedData {
        signature,
        data,
        nonce,
    }
}

#[get("/sign/<target>")]
fn ed25519_sign(target: &str) -> Json<SignedData> {
    let uuid = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc();
    let timestamp = now
        .format(&time::format_description::well_known::Iso8601::DEFAULT)
        .unwrap();
    let data_to_encode = EncodedData {
        target,
        timestamp: timestamp.as_str(),
        uuid: uuid.as_str(),
    };
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let key = read_private_key();
    Json(sign(data_to_encode, key, nonce))
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

#[cfg(test)]
mod tests {
    use crate::{sign, EncodedData, SignedData};
    use aes_gcm::aead::Nonce;
    use aes_gcm::Aes256Gcm;
    use ed25519_dalek::pkcs8::DecodePrivateKey;
    use ed25519_dalek::SigningKey;
    use spectral::assert_that;
    use std::num::NonZeroU8;
    use time::format_description::well_known::iso8601::{Config, EncodedConfig, TimePrecision};
    use time::format_description::well_known::Iso8601;
    use time::macros::datetime;
    use time::UtcOffset;

    fn private_key(pem: &str) -> SigningKey {
        SigningKey::from_pkcs8_pem(pem).unwrap()
    }

    const FORMAT: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: NonZeroU8::new(3),
        })
        .encode();

    #[test]
    fn encodes_data_given_by_specifications_correctly() {
        let timestamp = datetime!(2023-07-19 09:58:01.964 +2)
            .to_offset(UtcOffset::UTC)
            .format(&Iso8601::<FORMAT>)
            .unwrap();
        println!("{timestamp}");
        let signed_data = sign(
            EncodedData {
                target: "eo-150-1337",
                timestamp: timestamp.as_str(),
                uuid: "a38dbc68-8305-44ec-af18-d42f1f7d5fdc",
            },
            private_key(
                "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIP2nQ8utZvjI6uZx+ruN6B+lKdajeI1LZuxLfrD3zrqH
-----END PRIVATE KEY-----",
            ),
            Nonce::<Aes256Gcm>::from_slice(&[
                0x5F, 0x69, 0x28, 0xB5, 0x9D, 0xF0, 0x1C, 0x76, 0x65, 0x4D, 0xF4, 0x5D,
            ])
            .clone(),
        );
        assert_that!(signed_data).is_equal_to(SignedData {
            data: "%2BPHZKrp64EWAlLvDLI6Yhl0oaY42I3Y8WMuPPC0ErS5IreNMAQGu9XH6Ax%2FpcL7aRyRmbPZYRhpQEPauiUJf2mBGnEfIiTF%2F15vB9gL9DFQtuGZ5OYG3LLe0XuWsuDIzhiOkY7zKjvgWSd4MfRShx8QXBNCi08ynK0WYtms%3D".to_string(),
            nonce:"X2kotZ3wHHZlTfRd".to_string(),
            signature:"jMN0o5b%2FJbCL1OH3pzjGafVaQS72vPCz%2F3ZxSEOG2CurSmWQxQSlYjYpP0%2Fh8JjaqRSXfUDKNCSE2S6H1eiiBw%3D%3D".to_string()
        })
    }
}
