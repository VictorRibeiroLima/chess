use actix::{Addr, Recipient};
use uuid::Uuid;

use crate::{con::Con, messages::result::ResultMessage};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Client {
    id: Uuid,
    result_addr: Recipient<ResultMessage>,
}

impl Client {
    pub fn new(id: Uuid, addr: Addr<Con>) -> Self {
        let result_addr = addr.clone().recipient::<ResultMessage>();
        Self { id, result_addr }
    }

    pub fn result_addr(&self) -> &Recipient<ResultMessage> {
        &self.result_addr
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
