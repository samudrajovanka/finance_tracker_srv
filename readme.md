# Finance Tracker Server

A robust backend API for personal finance management built with Rust and Actix-web.

## 🛠️ Tech Stack

- **Framework**: [Actix-web](https://actix.rs/)
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) for compile-time checked queries
- **Runtime**: [Tokio](https://tokio.rs/) for async runtime

## 📋 Prerequisites

- Rust 1.70+ 
- PostgreSQL 12+

## 🔧 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/samudrajovanka/finance_tracker_srv
   cd finance-tracker-srv
   ```

2. **Set up environment variables**
   Create a `.env` file in the root directory with example from `.env.example`

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Set up the database**
   ```bash
   # Install SQLx CLI
   cargo install sqlx-cli

   # Run migrations
   sqlx migrate run
   ```

5. **Seed the database (optional)**
   ```bash
   cargo run --bin seed
   ```

## 🚀 Running the Server

### Development
```bash
# Basic run
cargo run

# Hot reload development (recommended)
# Install watchexec first: cargo install watchexec-cli
watchexec -e rs -r cargo run
```

The server will start on `http://localhost:8080` by default.

## 📁 Project Structure

```
finance-tracker-srv/
├── migrations/                # Database migration files
│   └── *.sql                  # Migration scripts
├── src/                       # Source code
│   ├── main.rs                # Application entry point
│   ├── bin/                   # Binary executables
│   │   └── seed/              # Database seeding scripts
│   ├── config/                # Configuration modules
│   ├── handlers/              # Global HTTP handlers
│   ├── routes/                # Route definitions
│   ├── modules/               # Feature modules (modular architecture)
│   │   ├── some_module/       # Some module
│   │   │   ├── handlers/      # Module-specific handlers
│   │   │   ├── models/        # Module-specific models
│   │   │   ├── repositories/  # Module-specific repositories
│   │   │   ├── services/      # Module business logic
│   │   │   ├── route.rs       # Module routes
│   │   │   └── mod.rs
│   │   └── mod.rs
│   └── utils/                 # Shared utilities
│       ├── errors/            # Error handling
│       ├── helpers/           # Helper functions
│       └── mod.rs
├── target/                    # Compiled output (generated)
├── Cargo.toml                 # Project configuration
├── Cargo.lock                 # Dependency lock file
└── .env                       # Environment variables
```
