use ed25519_dalek::{
    PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH, Signature, Signer, SigningKey, Verifier, VerifyingKey,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::TicketError;

#[derive(Serialize, Deserialize, Debug)]
enum TicketStatus {
    Unused,
    Used,
    Cancelled,
}

// struct Event {
// 	// TODO-Done: use uuid for id
// 	// TODO: use date type or crate for date
// 	// TODO: use time type/crate for time
// 	id: Uuid,
// 	name: String,
// 	date: String,
// 	time: String,
// 	location: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Ticket {
    // TODO-Done: use uuid for id
    // TODO-Done: use encrypted data for signature
    // TODO: use uuid for event or Event Struct called by uuid
    pub id: Uuid,
    pub event: String,
    pub price: f32,
    status: TicketStatus,
    pub public_key: [u8; PUBLIC_KEY_LENGTH],
    #[serde(with = "serde_bytes")]
    signature: [u8; SIGNATURE_LENGTH],
}

impl Ticket {
    pub fn new(event: String, price: f32) -> (Self, SigningKey) {
        let id: Uuid = Uuid::new_v4();

        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let message_string: String = format!("{id}{price}{event}");
        let message: &[u8] = message_string.as_bytes();

        let public_key = signing_key.verifying_key().to_bytes();
        let signature = signing_key.sign(message).to_bytes();

        (
            Self {
                id,
                event,
                price,
                status: TicketStatus::Unused,
                public_key,
                signature,
            },
            signing_key,
        )
    }

    pub fn verify(&self) -> Result<bool, TicketError> {
        let verifying_key: VerifyingKey = VerifyingKey::from_bytes(&self.public_key)
            .map_err(|_err| TicketError::CryptoError("Error getting Public Key".to_string()))?;
        let signature: Signature = Signature::try_from(self.signature)
            .map_err(|_err| TicketError::CryptoError("Error getting Signature".to_string()))?;
        let message_string: String = format!("{}{}{}", self.id, self.price, self.event);

        match verifying_key.verify(message_string.as_bytes(), &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn burn_ticket(mut self) -> Result<Self, String> {
        match self.verify() {
            Ok(value) => {
                if value {
                    match self.status {
                        TicketStatus::Unused => {
                            self.status = TicketStatus::Used;
                            // Ok("ticket has been successfully burned".to_string())
                            Ok(self)
                        }
                        TicketStatus::Used => Err("Ticket has already been used!".to_string()),
                        TicketStatus::Cancelled => {
                            Err("Event has been cancelled. Ticket is invalid!".to_string())
                        }
                    }
                } else {
                    Err("Ticket is invalid!".to_string())
                }
            }
            Err(err) => Err(format!("{}", err)),
        }
    }
}
