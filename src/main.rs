use clap::{Parser, Subcommand};
use ed25519_dalek::SigningKey;
use std::{
    io::{Write, stdin, stdout},
    process,
};
use ticket_validator::ticket_lib::Ticket;
use uuid::Uuid;

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

#[derive(Parser)]
#[command(
    name = "Ticket Validation Cli",
    version = "0.0.1",
    about = "A tool to create and validate event tickets",
    long_about = None
)]
struct TicketValidationCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a new ticket
    Create { name: String, price: f32 },

    /// Scans a ticket and burns it up if unsed
    Scan { ticket_uuid: Uuid },
}

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
            let new_ticket = Ticket::new("ut2345hhh".to_string(), 32.00);

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

fn create_ticket<'a>(
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

fn scan_ticket(ticket_uuid: Uuid, db: &mut PickleDb) -> Result<String, String> {
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
            process::exit(1);
        }
    } else {
        Err("\nCould not retrieve ticket!".to_string())
    }
}
