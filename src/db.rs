use crate::errors::TicketError;
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
) -> Result<String, TicketError> {
    // GIT: added checks to see if ticket exists before creation
    if !db.exists(format!("{}", ticket.0.id).as_str()) {
        if let Ok(()) = db.set(format!("{}", ticket.0.id).as_str(), &ticket.0) {
            db.dump().map_err(|_err| {
                TicketError::DatabaseError("\nCould not save ticket".to_string())
            })?;
            key_db
                .set(format!("{}", ticket.0.id).as_str(), &ticket.1)
                .map_err(|_err| {
                    TicketError::DatabaseError("\nCould not save private key".to_string())
                })?;
            key_db.dump().map_err(|_err| {
                TicketError::DatabaseError("\nCould not save private key".to_string())
            })?;

            // return Err("\nCould not save Signing Key");

            Ok(format!(
                "\nTicket ID: {} Successfully Created!\n\n",
                ticket.0.id
            ))
        } else {
            Err(TicketError::DatabaseError(
                "\nCould not save ticket".to_string(),
            ))
        }
    } else {
        Err(TicketError::DatabaseError(
            "\nTicket with that id already exist!".to_string(),
        ))
    }
}

pub fn scan_ticket(ticket_uuid: Uuid, db: &mut PickleDb) -> Result<String, String> {
    if let Some(ticket) = db.get::<Ticket>(
        &ticket_uuid
            .hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ) {
        let mut user_choice = String::new();

        'input: loop {
            print!("\nDo you want to use the ticket? (y/n): ");
            stdout().flush().unwrap();
            stdin().read_line(&mut user_choice).unwrap();
            // println!("you selected {user_choice}");

            if user_choice.trim().to_lowercase() == "y" {
                match ticket.burn_ticket() {
                    Ok(nticket) => {
                        if let Ok(()) = db.set(format!("{}", nticket.id).as_str(), &nticket) {
                            db.dump()
                                .map_err(|_err| "\nError saving database".to_string())?;
                            return Ok("\nTicket Used Successfully!".to_string());
                        } else {
                            return Err("\nError updating ticket".to_string());
                        }
                    }
                    Err(err) => return Err(format!("\nError updating ticket: {}", err)),
                }
            } else if user_choice.trim().to_lowercase() == "n" {
                println!("\n\n\n");
                // std::process::exit(1);
                break 'input Err("\nUser Exited CLI".to_string());
            }
        }
    } else {
        Err("\nCould not retrieve ticket!".to_string())
    }
}
