pub mod clients;
pub mod in_memory_database;
pub mod transactions;

use rusqlite::Connection;

pub trait InitializeDB {
    ///initializes a sqlite database with 3 tables, clients, aliases and transactions
    fn initialize(&mut self) -> Result<(), rusqlite::Error>;
}

impl InitializeDB for Connection {
    fn initialize(&mut self) -> Result<(), rusqlite::Error> {
        self.execute(
            "
        CREATE TABLE IF NOT EXISTS clients (
            uid INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            balance INTEGER NOT NULL DEFAULT 0,
            detail TEXT
        );",
            [],
        )?;

        self.execute(
            "
        CREATE TABLE IF NOT EXISTS transactions (
            tid INTEGER PRIMARY KEY NOT NULL,
            uid INTEGER,
            date INTEGER NOT NULL,
            price INTEGER NOT NULL,
            payment INTEGER NOT NULL DEFAULT 0,
            detail TEXT,

            FOREIGN KEY (uid) REFERENCES clients (uid)
                ON UPDATE CASCADE
                ON DELETE SET NULL
        );",
            [],
        )?;

        self.execute(
            "
        CREATE TABLE IF NOT EXISTS aliases (
            aid INTEGER PRIMARY KEY NOT NULL,
            uid INTEGER NOT NULL,
            alias TEXT NOT NULL,

            FOREIGN KEY (uid) REFERENCES clients (uid)
                ON UPDATE CASCADE
                ON DELETE CASCADE
        );",
            [],
        )?;

        Ok(())
    }
}
