use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(
    name = "Ticket Validation Cli",
    version = "0.0.1",
    about = "A tool to create and validate event tickets",
    long_about = None
)]
pub struct TicketValidationCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new ticket
    Create { name: String, price: f32 },

    /// Scans a ticket and burns it up if unsed
    Scan { ticket_uuid: Uuid },
}
