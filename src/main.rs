use std::error::Error;
use std::time::UNIX_EPOCH;
use crate::database::users::UserDB;
use crate::database::InitializeDB;
use crate::database::transactions::TransactionsDB;

mod database;

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = rusqlite::Connection::open("./db.db")?;

    conn.initialize()?;

    conn.add_user("lul", "1")?;
    let c = conn.get_users(
        database::users::Ufilter::new().with_name("lul".to_string()),
    )?
    .pop()
    .unwrap();

    let time = UNIX_EPOCH.elapsed().unwrap().as_secs();
    conn.add_transaction(
        Some(&c),
        1000,
        400,
        "pague weeee".to_string(),
        time,
    )?;

    conn.update_user_balance(&c)?;
    print!(
        "{:?}",
        conn.get_users(
            database::users::Ufilter::new().with_name("lul".to_string())
        )?
        .pop()
        .unwrap()
    );

    Ok(())
}
