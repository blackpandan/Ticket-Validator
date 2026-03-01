use std::fmt;

#[derive(Debug)]
pub enum TicketError {
    InvalidTicket(String),
    DatabaseError(String),
    CryptoError(String),
}

impl fmt::Display for TicketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TicketError::InvalidTicket(ref err) => write!(f, "InvalidTicket Error: {}", err),
            TicketError::DatabaseError(ref err) => write!(f, "Database Error: {}", err),
            TicketError::CryptoError(ref err) => write!(f, "Encrypting Ticket Error: {}", err),
        }
    }
}
