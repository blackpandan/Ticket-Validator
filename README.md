# 🎟️ TICKET VALIDATOR CLI (RUST)
---------------------------------
This cli tool allows creation and validation of ticket for events.

A Rust command-line application for creating, validating, and burning event tickets.  
This project reflects my current stage in learning Rust and is the first step in my journey toward blockchain development.

The setup is done using cargo, you can also use the exe from the release , you'll need to install rust if it's not available

---

## Features
- Create tickets with unique UUIDs
- Sign and verify tickets using `ed25519-dalek`
- Validate tickets once and burn them after scanning
- Store tickets with `pickledb`
- Simple CLI commands: `create`, `scan`, `help`

---

## Tech Stack
- **Language**: Rust
- **Libraries**:
  - [`uuid`](https://docs.rs/uuid/) for ticket IDs
  - [`ed25519-dalek`](https://docs.rs/ed25519-dalek/) for signing
  - [`pickledb`](https://crates.io/crates/pickledb) for storage
  - [`clap`](https://docs.rs/clap/) for CLI parsing

---

## Example Usage
```bash
# Create a new ticket
cargo run -- create "Concert"

# Scan a ticket
cargo run -- scan <ticket_id>


```

---

## My Developer Roadmap
This project is part of a four-month plan to grow into blockchain development:

- **Month 1:** Rust Mastery → CLI ticket validator (this project)
- **Month 2:** Solana Core → Anchor framework, PDAs, cNFTs (ticket minter for 100k seats)
- **Month 3:** NEAR UX → Mobile-first ticketing app with FastAuth and meta-transactions
- **Month 4:** Integration → Substrate + Octopus Network for cross-chain ticketing

---

# Why This Project Matters
- Builds a foundation in Rust with traits, error handling, and modular design
- Applies cryptography concepts in a practical way
- Sets the stage for blockchain integration
- Shows my ability to learn quickly and apply concepts to working code

---

## Usage with cargo
**cargo run --** {COMMAND}


## Usage with exe
**ticket_validator.exe** {COMMAND}


## Commmands
  - **create**  Creates a new ticket
  - **scan**    Scans a ticket and burns it up if unsed
  - **help**    Print this message or the help of the given subcommand(s)

## Options
  - **-h, --help**     Print help
  - **-V, --version**  Print version

---

