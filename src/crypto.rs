use ed25519_dalek::{Signature, Signer, SigningKey};
use hkdf::Hkdf;
use sha2::Sha256;
use std::env;
use uuid::Uuid;

use crate::errors::TicketError;

pub fn generate_key(ticket_id: Uuid) -> Result<SigningKey, TicketError> {
    // Getting the seed from env
    let master_seed: String = match dotenvy::dotenv() {
        Ok(_) => {
            if let Ok(m_seed) = env::var("MASTER_SEED") {
                m_seed
            } else {
                return Err(TicketError::CryptoError(
                    "Could not get seed from env".into(),
                ));
            }
        }
        Err(_) => {
            return Err(TicketError::CryptoError("Could not load env file".into()));
        }
    };
    let master_seed: &[u8] = master_seed.as_bytes();

    let salt: Option<&[u8]> = Some("ticket-validator-v1".as_bytes());
    let info: String = format!("ticket-id:{}", ticket_id);
    let info: &[u8] = info.as_bytes();
    let mut okm = [0u8; 32];

    let hk = Hkdf::<Sha256>::new(salt, master_seed);

    match hk.expand(info, &mut okm) {
        Ok(_) => Ok(SigningKey::from_bytes(&okm)),
        Err(e) => Err(TicketError::CryptoError(format!("{:?}", e))),
    }
}

pub fn sign_message(message: &[u8], ticket_id: Uuid) -> Result<Signature, TicketError> {
    let signing_key: SigningKey = generate_key(ticket_id)?;

    Ok(signing_key.sign(message))
}

pub fn verify_signature(
    message: &[u8],
    signature: Signature,
    ticket_id: Uuid,
) -> Result<bool, TicketError> {
    let signing_key: SigningKey = generate_key(ticket_id)?;

    if signing_key.verify(message, &signature).is_ok() {
        Ok(true)
    } else {
        Err(TicketError::CryptoError("Error Verifying Key".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use serial_test::serial;

    #[fixture]
    fn setup() -> (Uuid, SigningKey) {
        let id: Uuid = Uuid::new_v4();

        dotenvy::dotenv().expect("Could not load env file");

        let env_kv = env::var("MASTER_SEED");

        let ms_kv: String = match env_kv {
            Ok(key) => key,
            Err(err) => {
                eprintln!("{:?}", err);
                panic!();
            }
        };

        let info = format!("ticket-id:{}", id);

        let hk = Hkdf::<Sha256>::new(Some("ticket-validator-v1".as_bytes()), ms_kv.as_bytes());
        let mut okm: [u8; 32] = [0u8; 32];
        hk.expand(info.as_bytes(), &mut okm)
            .expect("Err Saving to OKM");

        let signing_key = SigningKey::from_bytes(&okm);

        (id, signing_key)
    }

    #[rstest]
    #[serial]
    fn test_sign_message(setup: (Uuid, SigningKey)) {
        let (id, signing_key) = setup;
        let message: &[u8] = "this is a test message".as_bytes();

        let test_signed: Signature = signing_key.sign(message);

        match sign_message(message, id) {
            Ok(signed) => assert!(signed == test_signed),
            Err(e) => println!("{:?}", e),
        };
    }

    #[rstest]
    #[serial]
    fn test_verify_message(setup: (Uuid, SigningKey)) {
        let (id, signing_key) = setup;

        let price: f32 = 500.43;
        let event: &str = "Tested Event";
        let message_string: String = format!("{id}{price}{event}");
        let message: &[u8] = message_string.as_bytes();

        let signature = signing_key.sign(message);
        assert!(verify_signature(message, signature, id).is_ok_and(|is_verified| is_verified))
    }
}
