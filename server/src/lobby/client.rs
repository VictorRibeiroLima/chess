use actix::{Addr, Recipient};
use uuid::Uuid;

use crate::{
    con::Con,
    messages::{success::SuccessMessage, ErrorMessage, StringMessage},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Client {
    id: Uuid,
    string_addr: Recipient<StringMessage>,
    error_addr: Recipient<ErrorMessage>,
    success_addr: Recipient<SuccessMessage>,
}

impl Client {
    pub fn new(id: Uuid, addr: Addr<Con>) -> Self {
        let string_addr = addr.clone().recipient::<StringMessage>();
        let error_addr = addr.clone().recipient::<ErrorMessage>();
        let success_addr = addr.recipient::<SuccessMessage>();
        Self {
            id,
            string_addr,
            error_addr,
            success_addr,
        }
    }

    pub fn _string_addr(&self) -> &Recipient<StringMessage> {
        &self.string_addr
    }

    pub fn error_addr(&self) -> &Recipient<ErrorMessage> {
        &self.error_addr
    }

    pub fn success_addr(&self) -> &Recipient<SuccessMessage> {
        &self.success_addr
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
