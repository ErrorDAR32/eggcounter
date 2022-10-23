use std::collections::{BTreeSet};
use std::error::Error;
use crate::database::clients::{Alias, Client, ClientError, Ufilter};
use crate::database::transactions::Transaction;
use crate::ClientDB;

pub struct InMemDB {
    last_uid: u64,
    clients: BTreeSet<Client>,
    aliases: BTreeSet<Alias>,
    transactions: BTreeSet<Transaction>
}
impl InMemDB {
    pub fn get_next_uid(&mut self) -> u64 {
        self.last_uid += 1;
        self.last_uid
    }
}

impl ClientDB for InMemDB {
    fn add_client(&mut self, name: &str, detail: &str) -> Result<(), ClientError> {
        let uid = self.get_next_uid();
        self.clients.insert(Client {
            cid: uid,
            name: name.to_string(),
            detail: Some(detail.to_string()),
            balance: 0});
        Ok(())
    }

    fn update_client(&mut self, c: &Client) -> Result<(), ClientError> {
        let client = match self.clients.take(c) {
            None => {return Err(ClientError::QueryError)}
            Some(c) => {c}
        };
        Ok(())
    }

    fn remove_client(self: &mut InMemDB, uid: u64) -> Result<(), ClientError> {
        todo!()
    }

    fn get_clients(&self, filter: Ufilter) -> Result<Vec<Client>, ClientError> {
        todo!()
    }

    fn add_alias<'a>(self: &mut InMemDB, client: &Client, alias: &'a str) -> Result<(), ClientError> {
        todo!()
    }

    fn get_aliases(&self, client: &Client) -> Result<Vec<Alias>, ClientError> {
        todo!()
    }

    fn remove_alias(self: &mut InMemDB, aliasid: u64) -> Result<(), ClientError> {
        todo!()
    }

    fn update_alias(self: &mut InMemDB, alias: &Alias) -> Result<(), ClientError> {
        todo!()
    }

    fn update_client_balance(self: &mut InMemDB, u: &Client) -> Result<(), ClientError> {
        todo!()
    }

    fn update_client_balance_delta(self: &mut InMemDB, uid: u64, balance_delta: i64) -> Result<(), ClientError> {
        todo!()
    }
}