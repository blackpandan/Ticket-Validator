use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

/// local import
use ticket_validator::{
    cli::{Commands, TicketValidationCli},
    db::{create_ticket, scan_ticket},
    ticket::Ticket,
};

fn main() {
    let cli = TicketValidationCli::parse();
    let mut db = match PickleDb::load(
        "tickets.db",
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    ) {
        Ok(existing_db) => existing_db,
        Err(_) => PickleDb::new(
            "tickets.db",
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        ),
    };

    let mut key_db = match PickleDb::load(
        "keys.db",
        PickleDbDumpPolicy::DumpUponRequest,
        SerializationMethod::Json,
    ) {
        Ok(existing_db) => existing_db,
        Err(_) => PickleDb::new(
            "keys.db",
            PickleDbDumpPolicy::DumpUponRequest,
            SerializationMethod::Json,
        ),
    };

    println!("\n\n-------------------------------------------------------------------------");
    println!("\n    TICKET VALIDATOR");
    println!("\n--------------------------------------------------------------------------\n\n");

    match &cli.command {
        Commands::Create { name, price } => {
            println!("'Creating Ticket!' -> Ticket {{ name: {name}, price: {price} }}");
            let new_ticket = Ticket::new(name.clone(), *price);

            let new_ticket = create_ticket(new_ticket, &mut db, &mut key_db);

            match new_ticket {
                Ok(success_message) => println!("{success_message}\n\n"),
                Err(error_message) => eprintln!("{error_message}\n\n"),
            }
        }

        Commands::Scan { ticket_uuid } => {
            println!("'Ticket scanning started!' -> Ticket UUID: {ticket_uuid}");
            let gotten_ticket = scan_ticket(*ticket_uuid, &mut db);

            match gotten_ticket {
                Ok(message) => println!("\n\nCOMPLETED: {}\n\n\n", message.trim()),
                Err(err) => eprintln!("{}\n\n", err),
            }
        }
    }
}
