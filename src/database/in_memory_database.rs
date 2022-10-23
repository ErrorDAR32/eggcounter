use std::collections::BTreeSet;

use crate::database::clients::{Alias, Client, ClientError, ClientFilter};
use crate::database::transactions::Transaction;
use crate::ClientDB;

#[derive(Debug)]
pub struct InMemDB {
    last_uid: u64,
    clients: BTreeSet<Client>,
    aliases: BTreeSet<Alias>,
    transactions: BTreeSet<Transaction>,
}
impl InMemDB {
    pub fn get_next_uid(&mut self) -> u64 {
        self.last_uid += 1;
        self.last_uid
    }
    pub fn new() -> InMemDB {
        InMemDB {
            last_uid: 0,
            clients: BTreeSet::new(),
            aliases: BTreeSet::new(),
            transactions: BTreeSet::new(),
        }
    }
}

impl ClientDB for InMemDB {
    fn add_client(&mut self, name: &str, detail: &str) -> Result<(), ClientError> {
        let uid = self.get_next_uid();
        self.clients.insert(Client {
            cid: uid,
            name: name.to_string(),
            detail: Some(detail.to_string()),
            balance: 0,
        });
        Ok(())
    }

    fn update_client(&mut self, c: &Client) -> Result<(), ClientError> {
        self.clients.replace(c.clone());

        Ok(())
    }

    fn remove_client(self: &mut InMemDB, cid: u64) -> Result<(), ClientError> {
        self.clients.retain(|c| c.uid() != cid);
        Ok(())
    }

    fn get_clients(&self, filter: ClientFilter) -> Result<Vec<Client>, ClientError> {
        let clients = self
            .clients
            .iter()
            .filter(|c| match filter.cid {
                None => true,
                Some(cid) => cid == c.cid,
            })
            .filter(|c| match &filter.name {
                None => true,
                Some(n) => n == &c.name,
            })
            .cloned()
            .collect();

        Ok(clients)
    }

    fn add_alias<'a>(
        self: &mut InMemDB,
        client: &Client,
        alias: &'a str,
    ) -> Result<(), ClientError> {
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

    fn update_client_balance_delta(
        self: &mut InMemDB,
        uid: u64,
        balance_delta: i64,
    ) -> Result<(), ClientError> {
        todo!()
    }
}
