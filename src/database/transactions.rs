use super::users::Client;
use rusqlite::Connection;
use sql_builder::prelude::*;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Transaction {
    detail: Option<String>,
    date: u64,
    uid: u64,
    tid: u64,
    price: i64,
    payment: i64,
}
#[derive(Debug)]
pub enum TransactionError {
    InvalidPaymentOnAnonClient,
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::InvalidPaymentOnAnonClient => {
                write!(f, "Invalid payment on anon client")
            }
        }
    }
}

impl Error for TransactionError {}

pub struct Tfilter {
    date_range: Option<(SystemTime, SystemTime)>,
    tid: Option<u64>,
    price_range: Option<(i64, i64)>,
    uid: Option<u64>,
}

pub fn add_transaction(
    conn: &Connection,
    client: Option<&Client>,
    price: i64,
    payment: i64,
    detail: String,
    date: u64,
) -> Result<(), Box<dyn Error>> {
    if client.is_none() && payment != price {
        return Err(Box::new(TransactionError::InvalidPaymentOnAnonClient));
    }

    let stm = SqlBuilder::insert_into("transactions")
        .fields(&["uid", "date", "price", "payment", "detail"])
        .values(&[
            client.map_or("NULL".to_string(), |c| c.uid().to_string()),
            date.to_string(),
            price.to_string(),
            payment.to_string(),
            quote(detail),
        ])
        .sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}

pub fn get_transactions(
    conn: &Connection,
    filter: Tfilter,
) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut stm = SqlBuilder::select_from("transactions");

    if let Some((min, max)) = filter.date_range {
        stm.and_where_ge("date", min.duration_since(UNIX_EPOCH)?.as_secs());
        stm.and_where_le("date", max.duration_since(UNIX_EPOCH)?.as_secs());
    }

    if let Some(uid) = filter.uid {
        stm.and_where_eq("uid", uid);
    }

    if let Some(tid) = filter.tid {
        stm.and_where_eq("tid", tid);
    }

    if let Some((min, max)) = filter.price_range {
        stm.and_where_ge("price", min);
        stm.and_where_le("price", max);
    }

    let mut sql_statement = conn.prepare(&stm.sql()?)?;

    let users = sql_statement.query_map([], |r| {
        Ok(Transaction {
            tid: r.get(0)?,
            uid: r.get(1)?,
            date: r.get(2)?,
            price: r.get(3)?,
            payment: r.get(4)?,
            detail: r.get(5)?,
        })
    })?;

    Ok(users
        .filter_map(|u| match u {
            Ok(a) => Some(a),
            Err(_) => None,
        })
        .collect())
}

pub fn update_transaction(conn: &Connection, t: &Transaction) -> Result<(), Box<dyn Error>> {
    let mut stm = SqlBuilder::update_table("transactions");

    if let Some(d) = &t.detail {
        stm.set("detail", quote(d));
    }

    stm.set("date", (Duration::from_secs(t.date)).as_secs())
        .set("uid", t.uid)
        .and_where_eq("tid", t.tid);

    conn.execute(&stm.sql()?, [])?;

    Ok(())
}

pub fn update_transaction_balance(
    conn: &Connection,
    tid: u64,
    balance_delta: i64,
) -> Result<(), Box<dyn Error>> {
    // todo! remember to update User balance
    let stm = SqlBuilder::update_table("transactions")
        .set("payment", "payment + ?".bind(&balance_delta))
        .and_where_eq("tid", tid)
        .sql()?;
    conn.execute(&stm, [])?;

    Ok(())
}

pub fn update_transaction_price(
    conn: &Connection,
    tid: u64,
    new_price: i64,
) -> Result<(), Box<dyn Error>> {
    // todo! remember to update User balance
    let stm = SqlBuilder::update_table("transactions")
        .set("price", &new_price)
        .and_where_eq("tid", tid)
        .sql()?;
    conn.execute(&stm, [])?;

    Ok(())
}

pub fn remove_transaction(conn: &Connection, tid: u64) -> Result<(), Box<dyn Error>> {
    // todo! remember to update User balance
    let stm = SqlBuilder::delete_from("transactions")
        .and_where_eq("tid", tid)
        .sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}
