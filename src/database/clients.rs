use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use rusqlite::Connection;
use sql_builder::prelude::*;
use crate::database::clients::ClientError::{InvalidQuery, QueryError};

#[derive(Debug, Eq, Ord)]
pub struct Client {
    pub name: String,
    pub detail: Option<String>,
    pub balance: i32,
    pub cid: u64,
}


impl Client {
    pub fn uid(&self) -> u64 {
        self.cid
    }
}

impl PartialEq<Self> for Client {
    fn eq(&self, other: &Self) -> bool {
        self.cid == other.cid
    }
}

impl PartialOrd<Self> for Client {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cid.cmp(&other.cid))
    }
}

pub struct Alias {
    alias: String,
    aid: u64,
    uid: u64,
}

pub struct Ufilter {
    name: Option<String>,
    uid: Option<u64>,
}

impl Ufilter {
    pub fn new() -> Ufilter {
        Ufilter {
            name: None,
            uid: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Ufilter {
        self.name = Some(name);
        self
    }

    pub fn with_uid(mut self, uid: u64) -> Ufilter {
        self.uid = Some(uid);
        self
    }
}

#[derive(Debug)]
pub enum ClientError {
    InvalidQuery,
    QueryError,
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ClientError {}

pub trait ClientDB {
    fn add_client(&mut self, name: &str, detail: &str) -> Result<(), ClientError>;

    fn update_client(&mut self, u: &Client) -> Result<(), ClientError>;

    fn remove_client(&mut self, uid: u64) -> Result<(), ClientError>;

    fn get_clients(&self, filter: Ufilter) -> Result<Vec<Client>, ClientError>;

    fn add_alias<'a>(&mut self, client: &Client, alias: &'a str) -> Result<(), ClientError>;

    fn get_aliases(&self, client: &Client) -> Result<Vec<Alias>, ClientError>;

    fn remove_alias(&mut self, aliasid: u64) -> Result<(), ClientError>;

    fn update_alias(&mut self, alias: &Alias) -> Result<(), ClientError>;

    fn update_client_balance(&mut self, u: &Client) -> Result<(), ClientError>;

    fn update_client_balance_delta(&mut self, uid: u64, balance_delta: i64)
        -> Result<(), ClientError>;
}

impl ClientDB for Connection {
    fn add_client(self: &mut Connection, name: &str, detail: &str) -> Result<(), ClientError> {
        let stm = SqlBuilder::insert_into("clients")
            .fields(&["name", "detail"])
            .values(&[&quote(name), &quote(detail)])
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;

        Ok(())
    }

    fn update_client(self: &mut Connection, u: &Client) -> Result<(), ClientError> {
        let mut stm_builder = SqlBuilder::update_table("clients");

        if let Some(d) = &u.detail {
            stm_builder.set("detail", d);
        }

        stm_builder.set("name", &u.name).and_where_eq("tid", u.cid);

        let stm = stm_builder.sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;

        Ok(())
    }

    fn remove_client(self: &mut Connection, uid: u64) -> Result<(), ClientError> {
        let stm = SqlBuilder::delete_from("clients")
            .and_where_eq("tid", uid)
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;

        Ok(())
    }

    fn get_clients(&self, filter: Ufilter) -> Result<Vec<Client>, ClientError> {
        //todo! search on aliases
        let mut stm_builder = SqlBuilder::select_from("clients");
        stm_builder.field("*");

        if let Some(n) = filter.name {
            stm_builder.and_where_eq("name", quote(n));
        }

        if let Some(uid) = filter.uid {
            stm_builder.and_where_eq("uid", uid);
        }
        let stm = stm_builder.sql().or(Err(InvalidQuery))?;

        let mut sql_statement = self.prepare(&stm).or(Err(InvalidQuery))?;

        let clients = sql_statement.query_map([], |r| {
                Ok(Client {
                    cid: r.get(0)?,
                    name: r.get(1)?,
                    balance: r.get(2)?,
                    detail: r.get(3)?,
                })
            }).or(Err(QueryError))?;

        Ok(clients
            .filter_map(|u| match u {
                Ok(a) => Some(a),
                Err(_) => None,
            })
            .collect())
    }

    fn add_alias(self: &mut Connection, client: &Client, alias: &str) -> Result<(), ClientError> {
        let stm = SqlBuilder::insert_into("aliases")
            .fields(&["uid", "alias"])
            .values(&[client.cid.to_string(), quote(alias)])
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;

        Ok(())
    }

    fn get_aliases(&self, client: &Client) -> Result<Vec<Alias>, ClientError> {
        let stm = SqlBuilder::select_from("aliases")
            .fields(&["aid", "alias"])
            .and_where_eq("uid", client.cid)
            .sql().or(Err(InvalidQuery))?;

        let mut sql_statement = self.prepare(&stm).or(Err(InvalidQuery))?;

        let aliases = sql_statement.query_map([], |r| {
            Ok(Alias {
                aid: r.get(0)?,
                alias: r.get(1)?,
                uid: client.cid,
            })
        }).or(Err(QueryError))?;

        Ok(aliases
            .filter_map(|u| match u {
                Ok(a) => Some(a),
                Err(_) => None,
            })
            .collect())
    }

    fn remove_alias(self: &mut Connection, aliasid: u64) -> Result<(), ClientError> {
        let stm = SqlBuilder::delete_from("aliases")
            .and_where_eq("aid", aliasid)
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;

        Ok(())
    }

    fn update_alias(self: &mut Connection, alias: &Alias) -> Result<(), ClientError> {
        let stm = SqlBuilder::update_table("aliases")
            .set("alias", &alias.alias)
            .and_where_eq("aid", alias.aid)
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;

        Ok(())
    }

    fn update_client_balance(self: &mut Connection, u: &Client) -> Result<(), ClientError> {
        let tp_stm = SqlBuilder::select_from("transactions")
            .field("SUM(price)")
            .and_where_eq("uid", u.cid)
            .subquery().or(Err(InvalidQuery))?;

        let pd_stm = SqlBuilder::select_from("transactions")
            .field("SUM(payment)")
            .and_where_eq("uid", u.cid)
            .subquery().or(Err(InvalidQuery))?;

        let stm = SqlBuilder::update_table("clients")
            .field("balance")
            .set("balance", &format!("{} - {}", pd_stm, tp_stm))
            .and_where_eq("uid", u.cid)
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;
        Ok(())
    }

    fn update_client_balance_delta(
        self: &mut Connection,
        uid: u64,
        balance_delta: i64,
    ) -> Result<(), ClientError> {
        let stm = SqlBuilder::update_table("clients")
            .field("balance")
            .set("balance", &format!("balance + {}", balance_delta))
            .and_where_eq("uid", uid)
            .sql().or(Err(InvalidQuery))?;

        self.execute(&stm, []).or(Err(QueryError))?;
        Ok(())
    }
}
