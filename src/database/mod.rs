pub mod transactions;
pub mod users;

use rusqlite::Connection;

///initializes a sqlite database with 3 tables, clients, aliases and transactions
pub fn initialize(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS clients (
            uid INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            balance INTEGER NOT NULL DEFAULT 0,
            detail TEXT
        );",
        [],
    )?;

    conn.execute(
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

    conn.execute(
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
