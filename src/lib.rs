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

struct Ticket {
    // TODO: use uuid for id
    // TODO: use uuid for event or Event Struct called by uuid
    // TODO: use encrypted data for signature
    id: String,
    event: String,
    price: f32,
    status: TicketStatus,
    signature: String,
}

impl Ticket {
    fn new(&self, event: String, price: f32) -> Self {
        Self {
            id: String::from("theidtee"),
            event,
            price,
            status: TicketStatus::Unused,
            signature: "yuagdhopoijfv".to_string(),
        }
    }

    fn verify(ticket_id: &str) {
        ticket_id.to_string();
    }
}
