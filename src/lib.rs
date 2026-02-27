pub mod ticket_lib {
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
    }
}
