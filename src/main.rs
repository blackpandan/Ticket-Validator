use clap::{Parser, Subcommand};
use ticket_validator::ticket_lib::Ticket;

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

    println!("TICKET VALIDATOR");

    match &cli.command {
        Commands::Create { name, price } => {
            println!("'create was used', name: {name}, price: {price}");
            let new_ticket: Ticket = Ticket::new("ut2345hhh".to_string(), 32.00);
        }
        Commands::Scan { ticket_uuid } => {
            println!("'scan was used', ticket_uuid: {ticket_uuid}");
        }
    }
}
