use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::{io, process};

// local import
use ticket_validator::{
    cli::{Commands, TicketValidationCli},
    db::{create_ticket, scan_ticket},
    errors::TicketError,
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

    println!("\n\n-------------------------------------------------------------------------");
    println!("\n    TICKET VALIDATOR");
    println!("\n--------------------------------------------------------------------------\n\n");

    match &cli.command {
        Commands::Create { name, price, venue } => {
            println!(
                "'Creating Ticket!' -> Ticket {{ name: {name}, price: {price}, venue: {venue} }}"
            );
            let new_ticket: Ticket = match Ticket::try_new(name.into(), price.into(), venue.into())
            {
                Ok(ticket) => {
                    println!("Ticket Successfully Created\n\n");
                    ticket
                }
                Err(error_message) => {
                    eprintln!("{error_message}\n\n");
                    process::exit(1);
                }
            };

            let new_ticket: Result<String, TicketError> = create_ticket(new_ticket, &mut db);

            match new_ticket {
                Ok(success_message) => println!("{success_message}\n\n"),
                Err(error_message) => eprintln!("{error_message}\n\n"),
            }
        }

        Commands::Scan { ticket_uuid } => {
            println!("'Ticket scanning started!' -> Ticket UUID: {ticket_uuid}");
            let stdin = io::stdin();
            let stdout = io::stdout();
            let gotten_ticket = scan_ticket(*ticket_uuid, &mut db, stdin.lock(), stdout);

            match gotten_ticket {
                Ok(message) => println!("\n\nCOMPLETED: {}\n\n\n", message.trim()),
                Err(err) => eprintln!("{}\n\n", err),
            }
        }

        Commands::List => {
            println!("Ticket Listing Started");
        }
    }
}
