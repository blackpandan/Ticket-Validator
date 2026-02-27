use clap::{Parser, Subcommand};
use ticket_validator::ticket_lib::Ticket;

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
    Scan { ticket_uuid: String },
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

    println!("TICKET VALIDATOR");

    match &cli.command {
        Commands::Create { name, price } => {
            println!("'create was used', name: {name}, price: {price}");
            let new_ticket: Ticket = Ticket::new("ut2345hhh".to_string(), 32.00);
            let new_ticket = create_ticket(new_ticket, &mut db);

            match new_ticket {
                Ok(success_message) => println!("{success_message}"),
                Err(error_message) => eprintln!("{error_message}"),
            }
        }

        Commands::Scan { ticket_uuid } => {
            println!("'scan was used', ticket_uuid: {ticket_uuid}");
            let gotten_ticket = scan_ticket("theidtee".to_string(), &mut db);

            match gotten_ticket {
                Ok(ticket) => println!("ticket retrieved: {:?}", ticket),
                Err(err) => eprintln!("{}", err),
            }
        }
    }
}

fn create_ticket(ticket: Ticket, db: &mut PickleDb) -> Result<String, &str> {
    println!("{}", ticket.id);
    if let Ok(()) = db.set(format!("{}", ticket.id).as_str(), &ticket) {
        db.dump().unwrap();
        Ok(format!("ticket: {} successfully created!", ticket.id))
    } else {
        Err("could not save ticket")
    }
}

fn scan_ticket(ticket_uuid: String, db: &mut PickleDb) -> Result<String, String> {
    if let Some(mut ticket) = db.get::<Ticket>(&ticket_uuid) {
        ticket.burn_ticket()
    } else {
        Err("could not retrieve ticket!".to_string())
    }
}
