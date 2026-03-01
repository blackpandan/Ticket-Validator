use crate::ticket::Ticket;

// External Imports
use ed25519_dalek::SigningKey;
use pickledb::PickleDb;
use std::io::{Write, stdin, stdout};
use uuid::Uuid;

pub fn create_ticket<'a>(
    ticket: (Ticket, SigningKey),
    db: &'a mut PickleDb,
    key_db: &'a mut PickleDb,
) -> Result<String, &'a str> {
    // GIT: added checks to see if ticket exists before creation
    if !db.exists(format!("{}", ticket.0.id).as_str()) {
        if let Ok(()) = db.set(format!("{}", ticket.0.id).as_str(), &ticket.0) {
            db.dump().unwrap();
            if let Ok(()) = key_db.set(format!("{}", ticket.0.id).as_str(), &ticket.1) {
                key_db.dump().unwrap();
            } else {
                return Err("\nCould not save Signing Key");
            }

            Ok(format!(
                "\nTicket ID: {} Successfully Created!\n\n",
                ticket.0.id
            ))
        } else {
            Err("\nCould not save ticket")
        }
    } else {
        Err("\nTicket with that id already exist!")
    }
}

pub fn scan_ticket(ticket_uuid: Uuid, db: &mut PickleDb) -> Result<String, String> {
    if let Some(ticket) = db.get::<Ticket>(
        &ticket_uuid
            .hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ) {
        let mut user_choice = String::new();

        print!("\nDo you want to use the ticket? (y/n): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut user_choice).unwrap();
        // println!("you selected {user_choice}");

        if user_choice.trim().to_lowercase() == "y" {
            match ticket.burn_ticket() {
                Ok(nticket) => {
                    if let Ok(()) = db.set(format!("{}", nticket.id).as_str(), &nticket) {
                        db.dump().unwrap();
                        Ok("\nTicket Used Successfully!".to_string())
                    } else {
                        Err("\nError updating ticket".to_string())
                    }
                }
                Err(err) => Err(format!("\nError updating ticket: {}", err)),
            }
        } else {
            println!("\n\n\n");
            std::process::exit(1);
        }
    } else {
        Err("\nCould not retrieve ticket!".to_string())
    }
}
