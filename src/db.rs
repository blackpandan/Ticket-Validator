use crate::errors::TicketError;
use crate::ticket::Ticket;

// External Imports
use ed25519_dalek::SigningKey;
use pickledb::PickleDb;
use std::io::{BufRead, Write};
use uuid::Uuid;

pub fn create_ticket(
    ticket: (Ticket, SigningKey),
    db: &mut PickleDb,
    key_db: &mut PickleDb,
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

pub fn scan_ticket<R, W>(
    ticket_uuid: Uuid,
    db: &mut PickleDb,
    mut reader: R,
    mut writer: W,
) -> Result<String, String>
where
    R: BufRead,
    W: Write,
{
    if let Some(ticket) = db.get::<Ticket>(
        ticket_uuid
            .hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    ) {
        let mut user_choice = String::new();

        'input: loop {
            write!(writer, "\nDo you want to use the ticket? (y/n): ")
                .map_err(|_| "Write Error".to_string())?;
            writer.flush().map_err(|_| "Flush Error".to_string())?;
            user_choice.clear();
            reader
                .read_line(&mut user_choice)
                .map_err(|_| "Read Error".to_string())?;
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
                writeln!(writer, "\n\n\n").map_err(|_| "Write Error".to_string())?;
                // std::process::exit(1);
                break 'input Err("\nUser Exited CLI".to_string());
            }
        }
    } else {
        Err("\nCould not retrieve ticket!".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use serial_test::serial;
    use std::io;

    const EVENT: &str = "Tested Event";
    const PRICE: f32 = 345.00;

    #[fixture]
    fn setup() -> (PickleDb, (Ticket, SigningKey)) {
        let db = PickleDb::new(
            "mem.db",
            pickledb::PickleDbDumpPolicy::NeverDump,
            pickledb::SerializationMethod::Json,
        );

        let ticket = Ticket::new(EVENT.to_string(), PRICE);

        (db, ticket)
    }

    #[fixture]
    fn key_db() -> PickleDb {
        PickleDb::new(
            "keymem.db",
            pickledb::PickleDbDumpPolicy::NeverDump,
            pickledb::SerializationMethod::Json,
        )
    }

    #[rstest]
    #[serial]
    fn test_create_ticket(setup: (PickleDb, (Ticket, SigningKey)), mut key_db: PickleDb) {
        let ticket: (Ticket, SigningKey) = setup.1.clone();
        let ticket_id = ticket.0.id;

        let mut db = setup.0;

        let r_ticket: Result<String, TicketError> = create_ticket(ticket, &mut db, &mut key_db);
        assert!(r_ticket.is_ok_and(|message| {
            message == format!("\nTicket ID: {} Successfully Created!\n\n", ticket_id)
        }));
    }

    #[rstest]
    #[serial]
    fn test_scan_ticket(setup: (PickleDb, (Ticket, SigningKey)), mut key_db: PickleDb) {
        let ticket: (Ticket, SigningKey) = setup.1.clone();
        let ticket_id: Uuid = setup.1 .0.id;
        let mut db = setup.0;

        // MOck INput for std
        let input = "y\n".as_bytes();
        let mut reader = io::Cursor::new(input);

        //Mock output
        let mut writer: Vec<u8> = Vec::new();

        let c_ticket: Result<String, TicketError> = create_ticket(ticket, &mut db, &mut key_db);
        assert!(c_ticket.is_ok_and(|message| {
            message == format!("\nTicket ID: {} Successfully Created!\n\n", ticket_id)
        }));

        let s_ticket: Result<String, String> =
            scan_ticket(ticket_id, &mut db, &mut reader, &mut writer);
        println!("{:?}", s_ticket);
        assert!(s_ticket.is_ok_and(|message| { message == "\nTicket Used Successfully!" }));
    }
}
