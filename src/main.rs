use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::{io, process};

// local import
use ticket_validator::{
    cli::{Commands, TicketValidationCli},
    db::{cancel_event, create_ticket, list_ticket, scan_ticket},
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

    println!(
                "\n\n-----------------------------------------------------------------------------------------"
            );
    println!("\n    TICKET VALIDATOR");
    println!(
                "\n-----------------------------------------------------------------------------------------\n\n"
            );

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

        Commands::List { filter } => {
            println!("Ticket Listing Started");
            let ticket_list: Vec<Ticket> = match list_ticket(&mut db) {
                Ok(tickets) => {
                    println!("\nTickets Retrieved Successfully\n");

                    match filter {
                        Some(flag) => {
                            let values: Vec<&str> = flag.trim().split(':').collect();

                            match values[0].to_lowercase().as_str() {
                                "event" => tickets
                                    .into_iter()
                                    .filter(|ticket| ticket.event.name.contains(values[1]))
                                    .collect(),
                                _ => tickets
                                    .into_iter()
                                    .filter(|ticket| ticket.event.name.contains(values[1]))
                                    .collect(),
                            }
                        }
                        _ => tickets,
                    }
                }
                Err(error_message) => {
                    eprintln!("\n{error_message}\n\n");
                    process::exit(1);
                }
            };

            println!(
                "\n\n-----------------------------------------------------------------------------------------"
            );
            println!("            ID -->                      Event Name,  Price,  Status");
            println!(
                "-----------------------------------------------------------------------------------------\n"
            );
            for ticket in ticket_list {
                println!("{}", ticket)
            }
            println!("\n\n\n")
        }

        Commands::Cancel { name } => {
            println!("\nCancelling Event...");
            println!("\n\nEvent Cancelling Started! -> Event Name: {}", name);
            println!("\n");
            let stdin = io::stdin();
            let stdout = io::stdout();
            let message = cancel_event(&mut db, name.to_string(), stdin.lock(), stdout);

            match message {
                Ok(message) => println!("\n\nCOMPLETED: {}\n\n\n", message.trim()),
                Err(err) => eprintln!("{}\n\n", err),
            }
        }
    }
}
