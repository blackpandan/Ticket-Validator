use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH,
};

use crate::errors::TicketError;

fn sign_message(message: &[u8]) -> Result<Signature, TicketError> {
    Err(TicketError::CryptoError("unimplemented!()".into()))
}

fn verify_signature(signature: Signature) -> Result<bool, TicketError> {
    Err(TicketError::CryptoError("unimplemented!()".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hkdf::Hkdf;
    use rstest::*;
    use serial_test::serial;
    use sha2::Sha256;
    use std::env;
    use uuid::Uuid;

    #[fixture]
    fn setup() -> (Uuid, SigningKey) {
        let id: Uuid = Uuid::new_v4();

        dotenvy::from_filename(".env.local").expect("Could not load env file");

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
        let (_id, signing_key) = setup;
        let message: &[u8] = "this is a test message".as_bytes();

        let test_signed: Signature = signing_key.sign(message);

        assert!(sign_message(message).is_ok_and(|signed| signed == test_signed));
    }

    #[rstest]
    #[serial]
    fn test_verify_message(setup: (Uuid, SigningKey)) {
        let (id, signing_key) = setup;

        let price: f32 = 500.43;
        let event: &str = "Tested Event";
        let message_string: String = format!("{id}{price}{event}");
        let message: &[u8] = message_string.as_bytes();

        //let public_key = signing_key.verifying_key().to_bytes();
        let signature = signing_key.sign(message);
        assert!(verify_signature(signature).is_ok_and(|is_verified| is_verified))
    }
}
