use crate::errors::TicketError;
use crate::ticket::Ticket;

// External Imports
use pickledb::PickleDb;
use std::io::{BufRead, Write};
use uuid::Uuid;

pub fn create_ticket(ticket: Ticket, db: &mut PickleDb) -> Result<String, TicketError> {
    // GIT: added checks to see if ticket exists before creation
    if !db.exists(format!("{}", ticket.id).as_str()) {
        if let Ok(()) = db.set(format!("{}", ticket.id).as_str(), &ticket) {
            db.dump().map_err(|_err| {
                TicketError::DatabaseError("\nCould not save ticket".to_string())
            })?;

            Ok(format!(
                "\nTicket ID: {} Successfully Created!\n\n",
                ticket.id
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
) -> Result<String, TicketError>
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
                .map_err(|err| TicketError::DatabaseError(format!("Write Error -> {}", err)))?;
            writer.flush().map_err(|err| {
                TicketError::DatabaseError(format!("User Input Error -> {}", err))
            })?;
            user_choice.clear();
            reader
                .read_line(&mut user_choice)
                .map_err(|err| TicketError::DatabaseError(format!("Read Error: {}", err)))?;
            // println!("you selected {user_choice}");

            if user_choice.trim().to_lowercase() == "y" {
                match ticket.burn_ticket() {
                    Ok(nticket) => match db.set(format!("{}", nticket.id).as_str(), &nticket) {
                        Ok(()) => {
                            db.dump().map_err(|err| {
                                TicketError::DatabaseError(format!(
                                    "\nError saving database -> {}",
                                    err
                                ))
                            })?;
                            return Ok("\nTicket Used Successfully!".to_string());
                        }
                        Err(err) => {
                            return Err(TicketError::DatabaseError(format!(
                                "\nError updating ticket -> {}",
                                err
                            )))
                        }
                    },
                    Err(err) => {
                        return Err(TicketError::DatabaseError(format!(
                            "\nError updating ticket: {}",
                            err
                        )))
                    }
                }
            } else if user_choice.trim().to_lowercase() == "n" {
                writeln!(writer, "\n\n\n")
                    .map_err(|err| TicketError::DatabaseError(format!("Write Error -> {}", err)))?;
                // std::process::exit(1);
                break 'input Err(TicketError::DatabaseError("\nUser Exited CLI".to_string()));
            }
        }
    } else {
        Err(TicketError::DatabaseError(
            "\nCould not retrieve ticket!".to_string(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use serial_test::serial;
    use std::io;

    const EVENT: &str = "Tested Event";
    const PRICE: &str = "345.00";
    const VENUE: &str = "earth-moon, milkyway";

    #[fixture]
    fn setup() -> (PickleDb, Ticket) {
        let db = PickleDb::new(
            "mem.db",
            pickledb::PickleDbDumpPolicy::NeverDump,
            pickledb::SerializationMethod::Json,
        );

        let ticket = Ticket::try_new(EVENT.to_string(), PRICE.into(), VENUE.into())
            .expect("error creating ticket");

        (db, ticket)
    }

    #[rstest]
    #[serial]
    fn test_create_ticket(setup: (PickleDb, Ticket)) {
        let ticket: Ticket = setup.1.clone();
        let ticket_id = ticket.id;

        let mut db = setup.0;

        let r_ticket: Result<String, TicketError> = create_ticket(ticket, &mut db);
        assert!(r_ticket.is_ok_and(|message| {
            message == format!("\nTicket ID: {} Successfully Created!\n\n", ticket_id)
        }));
    }

    #[rstest]
    #[serial]
    fn test_scan_ticket(setup: (PickleDb, Ticket)) -> Result<(), TicketError> {
        let ticket: Ticket = setup.1.clone();
        let ticket_id: Uuid = ticket.id;
        let mut db = setup.0;

        // MOck INput for std
        let input = "y\n".as_bytes();
        let mut reader = io::Cursor::new(input);

        //Mock output
        let mut writer: Vec<u8> = Vec::new();

        let message: String = create_ticket(ticket, &mut db)?;
        assert!(message == format!("\nTicket ID: {} Successfully Created!\n\n", ticket_id));

        let message: String = scan_ticket(ticket_id, &mut db, &mut reader, &mut writer)?;
        assert!(message == "\nTicket Used Successfully!");

        Ok(())
    }
}
