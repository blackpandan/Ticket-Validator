pub mod ticket_lib {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    enum TicketStatus {
        Unused,
        Used,
        Cancelled,
    }

    // struct Event {
    // 	// TODO: use uuid for id
    // 	// TODO: use date type or crate for date
    // 	// TODO: use time type/crate for time
    // 	id: String,
    // 	name: String,
    // 	date: String,
    // 	time: String,
    // 	location: String,
    // }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Ticket {
        // TODO: use uuid for id
        // TODO: use uuid for event or Event Struct called by uuid
        // TODO: use encrypted data for signature
        pub id: String,
        pub event: String,
        pub price: f32,
        status: TicketStatus,
        signature: String,
    }

    impl Ticket {
        pub fn new(event: String, price: f32) -> Self {
            Self {
                id: String::from("theidtee"),
                event,
                price,
                status: TicketStatus::Unused,
                signature: "yuagdhopoijfv".to_string(),
            }
        }

        pub fn verify(ticket_id: &str) {
            ticket_id.to_string();
        }

        pub fn burn_ticket(mut self) -> Result<Self, String> {
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
        }
    }
}
