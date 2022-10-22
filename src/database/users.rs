use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use rusqlite::Connection;
use sql_builder::prelude::*;

#[derive(Debug)]
pub struct Client {
    name: String,
    detail: Option<String>,
    balance: i32,
    uid: u64,
}

impl Client {
    pub fn uid(&self) -> u64 {
        self.uid
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
pub enum UserError {
    SqlInvalidQuery,
    SqlQueryError,
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for UserError {}

//todo: change dynamic errors for proper enums
pub trait UserDB {
    fn add_user(&self, name: &str, detail: &str) -> Result<(), Box<dyn Error>>;

    fn update_user(&self, u: &Client) -> Result<(), Box<dyn Error>>;

    fn remove_user(&self, uid: u64) -> Result<(), Box<dyn Error>>;

    fn get_users(&self, filter: Ufilter) -> Result<Vec<Client>, Box<dyn Error>>;

    fn add_alias<'a>(&self, client: &Client, alias: &'a str) -> Result<(), Box<dyn Error>>;

    fn get_aliases(&self, client: &Client) -> Result<Vec<Alias>, Box<dyn Error>>;

    fn remove_alias(&self, aliasid: u64) -> Result<(), Box<dyn Error>>;

    fn update_alias(&self, alias: &Alias) -> Result<(), Box<dyn Error>>;

    fn update_user_balance(&self, u: &Client) -> Result<(), Box<dyn Error>>;

    fn update_user_balance_delta(&self, uid: u64, balance_delta: i64)
        -> Result<(), Box<dyn Error>>;
}

impl UserDB for Connection {
    fn add_user(&self, name: &str, detail: &str) -> Result<(), Box<dyn Error>> {
        let stm = SqlBuilder::insert_into("clients")
            .fields(&["name", "detail"])
            .values(&[&quote(name), &quote(detail)])
            .sql()?;

        self.execute(&stm, [])?;

        Ok(())
    }

    fn update_user(&self, u: &Client) -> Result<(), Box<dyn Error>> {
        let mut stm = SqlBuilder::update_table("clients");

        if let Some(d) = &u.detail {
            stm.set("detail", d);
        }

        stm.set("name", &u.name).and_where_eq("tid", u.uid);

        self.execute(&stm.sql()?, [])?;

        Ok(())
    }

    fn remove_user(&self, uid: u64) -> Result<(), Box<dyn Error>> {
        let stm = SqlBuilder::delete_from("clients")
            .and_where_eq("tid", uid)
            .sql()?;

        self.execute(&stm, [])?;

        Ok(())
    }

    fn get_users(&self, filter: Ufilter) -> Result<Vec<Client>, Box<dyn Error>> {
        //todo! search on aliases
        let mut stm = SqlBuilder::select_from("clients");
        stm.field("*");

        if let Some(n) = filter.name {
            stm.and_where_eq("name", quote(n));
        }

        if let Some(uid) = filter.uid {
            stm.and_where_eq("uid", uid);
        }

        let mut sql_statement = self.prepare(&stm.sql()?)?;

        let users = sql_statement.query_map([], |r| {
            Ok(Client {
                uid: r.get(0)?,
                name: r.get(1)?,
                balance: r.get(2)?,
                detail: r.get(3)?,
            })
        })?;

        Ok(users
            .filter_map(|u| match u {
                Ok(a) => Some(a),
                Err(_) => None,
            })
            .collect())
    }

    fn add_alias<'a>(&self, client: &Client, alias: &'a str) -> Result<(), Box<dyn Error>> {
        let stm = SqlBuilder::insert_into("aliases")
            .fields(&["uid", "alias"])
            .values(&[client.uid.to_string(), quote(alias)])
            .sql()?;

        self.execute(&stm, [])?;

        Ok(())
    }

    fn get_aliases(&self, client: &Client) -> Result<Vec<Alias>, Box<dyn Error>> {
        let stm = SqlBuilder::select_from("aliases")
            .fields(&["aid", "alias"])
            .and_where_eq("uid", client.uid)
            .sql()?;

        let mut sql_statement = self.prepare(&stm)?;

        let aliases = sql_statement.query_map([], |r| {
            Ok(Alias {
                aid: r.get(0)?,
                alias: r.get(1)?,
                uid: client.uid,
            })
        })?;

        Ok(aliases
            .filter_map(|u| match u {
                Ok(a) => Some(a),
                Err(_) => None,
            })
            .collect())
    }

    fn remove_alias(&self, aliasid: u64) -> Result<(), Box<dyn Error>> {
        let stm = SqlBuilder::delete_from("aliases")
            .and_where_eq("aid", aliasid)
            .sql()?;

        self.execute(&stm, [])?;

        Ok(())
    }

    fn update_alias(&self, alias: &Alias) -> Result<(), Box<dyn Error>> {
        let stm = SqlBuilder::update_table("aliases")
            .set("alias", &alias.alias)
            .and_where_eq("aid", alias.aid)
            .sql()?;

        self.execute(&stm, [])?;

        Ok(())
    }

    fn update_user_balance(&self, u: &Client) -> Result<(), Box<dyn Error>> {
        let tp_stm = SqlBuilder::select_from("transactions")
            .field("SUM(price)")
            .and_where_eq("uid", u.uid)
            .subquery()?;

        let pd_stm = SqlBuilder::select_from("transactions")
            .field("SUM(payment)")
            .and_where_eq("uid", u.uid)
            .subquery()?;

        let stm = SqlBuilder::update_table("clients")
            .field("balance")
            .set("balance", &format!("{} - {}", pd_stm, tp_stm))
            .and_where_eq("uid", u.uid)
            .sql()?;

        self.execute(&stm, [])?;
        Ok(())
    }

    fn update_user_balance_delta(
        &self,
        uid: u64,
        balance_delta: i64,
    ) -> Result<(), Box<dyn Error>> {
        let stm = SqlBuilder::update_table("clients")
            .field("balance")
            .set("balance", &format!("balance + {}", balance_delta))
            .and_where_eq("uid", uid)
            .sql()?;

        self.execute(&stm, [])?;
        Ok(())
    }
}
