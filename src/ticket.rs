use std::str::FromStr;

use crate::{crypto, event::Event};
use ed25519_dalek::{Signer, SigningKey, SIGNATURE_LENGTH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errors::TicketError, price::Price};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    // TODO-Done: use uuid for id
    // TODO-Done: use encrypted data for signature
    // TODO: use uuid for event or Event Struct called by uuid
    pub id: Uuid,
    pub event: Event,
    pub price: Price,
    status: TicketStatus,
    //pub public_key: [u8; PUBLIC_KEY_LENGTH],
    #[serde(with = "serde_bytes")]
    signature: [u8; SIGNATURE_LENGTH],
}

impl Ticket {
    pub fn try_new(name: String, price_str: String, venue: String) -> Result<Self, TicketError> {
        let price: Price = Price::from_str(&price_str)?;

        let id: Uuid = Uuid::new_v4();
        let signing_key: SigningKey = crypto::generate_key(id)?;

        let event: Event = Event { name, venue };

        let message_string: String = format!("{id}{price}{event}");
        let message: &[u8] = message_string.as_bytes();

        let signature = signing_key.sign(message).to_bytes();

        Ok(Self {
            id,
            price,
            event,
            status: TicketStatus::Unused,
            signature,
        })
    }

    pub fn verify(&self) -> Result<bool, TicketError> {
        //    let verifying_key: VerifyingKey = VerifyingKey::from_bytes(&self.public_key)
        //        .map_err(|_err| TicketError::CryptoError("Error getting Public Key".to_string()))?;
        //    let signature: Signature = Signature::from_bytes(&self.signature);
        //    let message_string: String = format!("{}{}{}", self.id, self.price, self.event);
        //

        let message: String = format!("{}{}{}", self.id, self.price, self.event);
        let message: &[u8] = message.as_bytes();
        crypto::verify_signature(message, self.signature.into(), self.id)
    }

    pub fn burn_ticket(mut self) -> Result<Self, TicketError> {
        let message: String = format!("{}{}{}", self.id, self.price, self.event);
        let message: &[u8] = message.as_bytes();
        match crypto::verify_signature(message, self.signature.into(), self.id) {
            Ok(value) => {
                if value {
                    match self.status {
                        TicketStatus::Unused => {
                            self.status = TicketStatus::Used;
                            // Ok("ticket has been successfully burned".to_string())
                            Ok(self)
                        }
                        TicketStatus::Used => Err(TicketError::InvalidTicket(
                            "Ticket has already been used!".to_string(),
                        )),
                        TicketStatus::Cancelled => Err(TicketError::InvalidTicket(
                            "Event has been cancelled. Ticket is invalid!".to_string(),
                        )),
                    }
                } else {
                    Err(TicketError::InvalidTicket("Ticket is invalid!".to_string()))
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;
    const EVENT: &str = "Passe Entrant";
    const PRICE: &str = "90.58";
    const VENUE: &str = "earth-moon, milkyway";

    #[fixture]
    fn ticket() -> Ticket {
        Ticket::try_new(EVENT.to_string(), PRICE.into(), VENUE.into())
            .expect("Error Creating Ticket")
    }

    #[rstest]
    fn test_verify(ticket: Ticket) -> Result<(), TicketError> {
        let is_verified: bool = ticket.verify()?;
        assert!(is_verified);
        Ok(())
    }

    #[rstest]
    fn test_burn_ticket(ticket: Ticket) -> Result<(), TicketError> {
        let tik: Ticket = ticket.burn_ticket()?;
        assert_eq!(tik.status, TicketStatus::Used);
        Ok(())
    }
}
