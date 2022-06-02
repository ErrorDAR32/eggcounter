use std::error::Error;
use std::time::UNIX_EPOCH;

mod database;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = rusqlite::Connection::open("./db.db")?;

    database::initialize(&conn)?;

    database::users::add_user(&conn, "1", "lul")?;
    let c = database::users::get_users(&conn, database::users::Ufilter::new().with_name("1".to_string()))?.pop().unwrap();
    let time = UNIX_EPOCH.elapsed().unwrap().as_secs();
    database::transactions::add_transaction(&conn, Some(&c), 1000, 400, "pague weeee".to_string(), time)?;

    database::users::update_user_balance(&conn, &c)?;
    print!("{:?}", database::users::get_users(&conn, database::users::Ufilter::new().with_name("1".to_string()))?.pop().unwrap());

    Ok(())
}
