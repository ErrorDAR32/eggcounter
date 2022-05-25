use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rusqlite::Connection;
use super::users::Client;
use sql_builder::prelude::*;

pub struct Transaction {
    detail: Option<String>,
    date: u64,
    uid: u64,
    tid: u64,
    to_pay: i64,
    payed: i64,
}

pub struct Tfilter {
    date_range: Option<(SystemTime, SystemTime)>,
    tid: Option<u64>,
    amount_range: Option<(i64, i64)>,
    uid: Option<u64>,
}

pub fn add_transaction(conn: &Connection, client: Option<&Client>, to_pay: i64, payed: i64, detail: String, date: u64) -> Result<(), Box<dyn Error>> {
    let stm = SqlBuilder
    ::insert_into("transactions")
        .fields(&["uid", "date", "to_pay", "payed", "detail"])
        .values(&[
            client.map_or("NULL".to_string(), |c| c.uid().to_string()),
            date.to_string(),
            to_pay.to_string(),
            payed.to_string(),
            detail
        ]).sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}

pub fn get_transactions(conn: &Connection, filter: Tfilter) -> Result<Vec<Transaction>, Box<dyn Error>> {
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

    if let Some((min, max)) = filter.amount_range {
        stm.and_where_ge("to_pay", min);
        stm.and_where_le("to_pay", max);
    }

    let mut sql_statement = conn.prepare(&stm.sql()?)?;

    let users = sql_statement.query_map([], |r| {
        Ok(
            Transaction {
                tid: r.get(0)?,
                uid: r.get(1)?,
                date: r.get(2)?,
                to_pay: r.get(3)?,
                payed: r.get(4)?,
                detail: r.get(5)?,
            }
        )
    })?;

    Ok(
        users
            .filter_map(|u| match u {
                Ok(a) => Some(a),
                Err(_) => None
            })
            .collect()
    )
}

pub fn update_transaction(conn: &Connection, t: &Transaction) -> Result<(), Box<dyn Error>> {
    let mut stm = SqlBuilder::update_table("transactions");

    if let Some(d) = &t.detail {
        stm.set("detail", quote(d));
    }

    stm.set("date", (Duration::from_secs(t.date)).as_secs())
        .set("uid", t.uid)
        .and_where_eq("tid", t.tid);;


    conn.execute(&stm.sql()?, [])?;

    Ok(())
}

pub fn update_transaction_balance(conn: &Connection, tid: u64, balance_delta: i64) -> Result<(), Box<dyn Error>> { // todo! remember to update User balance
    let mut stm = SqlBuilder::update_table("transactions")
        .set("payed", "payed + ?".bind(&balance_delta))
        .and_where_eq("tid", tid).sql()?;
    conn.execute(&stm, [])?;

    Ok(())
}

pub fn update_transaction_price(conn: &Connection, tid: u64, new_price: i64) -> Result<(), Box<dyn Error>> { // todo! remember to update User balance
    let mut stm = SqlBuilder::update_table("transactions")
        .set("to_pay", &new_price)
        .and_where_eq("tid", tid).sql()?;
    conn.execute(&stm, [])?;

    Ok(())
}

pub fn remove_transaction<'a>(conn: &Connection, tid: u64) -> Result<(), Box<dyn Error>> { // todo! remember to update User balance
    let stm = SqlBuilder
    ::delete_from("transactions")
        .and_where_eq("tid", tid).sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}
