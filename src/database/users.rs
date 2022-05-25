use rusqlite::Connection;
use std::error::Error;
use sql_builder::prelude::*;

pub struct Client {
    name: String,
    detail: Option<String>,
    balance: i32,
    uid: u64,
}

impl Client {
    pub fn get_uid(&self) -> u64 { self.uid }
}

pub struct Alias {
    alias: String,
    aid: u64,
    uid: u64,
}

pub struct Ufilter<'a> {
    name: Option<&'a str>,
    uid: Option<u64>,
}

pub fn add_user(conn: &Connection, name: &str, detail: &str) -> Result<(), Box<dyn Error>> {
    let stm = SqlBuilder
    ::insert_into("clients")
        .fields(&["name", "detail"])
        .values(&[&quote(name), &quote(detail)]).sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}

pub fn update_user(conn: &Connection, u: &Client) -> Result<(), Box<dyn Error>> {
    let mut stm = SqlBuilder::update_table("clients");

    if let Some(d) = &u.detail {
        stm.set("detail", d);
    }

    stm.set("name", &u.name)
        .and_where_eq("tid", u.uid);

    conn.execute(&stm.sql()?, [])?;

    Ok(())
}

pub fn remove_user(conn: &Connection, uid: u64) -> Result<(), Box<dyn Error>> {
    let stm = SqlBuilder
    ::delete_from("clients")
        .and_where_eq("tid", uid).sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}

pub fn get_users(conn: &Connection, filter: Ufilter) -> Result<Vec<Client>, Box<dyn Error>> { //todo! search on aliases
    let mut stm = SqlBuilder::select_from("clients");
    stm.field("*");

    if let Some(n) = filter.name {
        stm.and_where_eq("name", quote(n));
    }

    if let Some(uid) = filter.uid {
        stm.and_where_eq("uid", uid);
    }

    let mut sql_statement = conn.prepare(&stm.sql()?)?;

    let users = sql_statement.query_map([], |r| {
        Ok(
            Client {
                uid: r.get(0)?,
                name: r.get(1)?,
                balance: r.get(2)?,
                detail: r.get(3)?,
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

pub fn add_alias<'a>(conn: &Connection, client: &Client, alias: &'a str) -> Result<(), Box<dyn Error>> {
    let stm = SqlBuilder
    ::insert_into("aliases")
        .fields(&["uid", "alias"])
        .values(&[client.uid.to_string(), quote(alias)]).sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}

pub fn get_aliases(conn: &Connection, client: &Client) -> Result<Vec<Alias>, Box<dyn Error>> {
    let mut stm = SqlBuilder
    ::select_from("aliases")
        .fields(&["aid", "alias"])
        .and_where_eq("uid", client.uid).sql()?;

    let mut sql_statement = conn.prepare(&stm)?;

    let aliases = sql_statement.query_map([], |r| {
        Ok(
            Alias {
                aid: r.get(0)?,
                alias: r.get(1)?,
                uid: client.uid,
            }
        )
    })?;

    Ok(
        aliases
            .filter_map(|u| match u {
                Ok(a) => Some(a),
                Err(_) => None
            })
            .collect()
    )

}

pub fn remove_alias<'a>(conn: &Connection, aliasid: u64) -> Result<(), Box<dyn Error>> {
    let stm = SqlBuilder
    ::delete_from("aliases")
        .and_where_eq("aid", aliasid).sql()?;

    conn.execute(&stm, [])?;

    Ok(())
}
