use crate::database::clients::ClientDB;
use crate::database::transactions::TransactionsDB;
use crate::database::InitializeDB;
use std::error::Error;
use std::time::UNIX_EPOCH;

mod database;

#[cfg(test)]
mod tests;

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = rusqlite::Connection::open("./db.db")?;

    conn.initialize()?;

    conn.add_client("lul", "1")?;
    let c = conn
        .get_clients(database::clients::ClientFilter::new().with_name("lul".to_string()))?
        .pop()
        .unwrap();

    let time = UNIX_EPOCH.elapsed().unwrap().as_secs();
    conn.add_transaction(Some(&c), 1000, 400, "pague weeee".to_string(), time)?;

    conn.update_client_balance(&c)?;
    print!(
        "{:?}",
        conn.get_clients(database::clients::ClientFilter::new().with_name("lul".to_string()))?
            .pop()
            .unwrap()
    );

    Ok(())
}
