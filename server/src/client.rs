use actix::Recipient;
use uuid::Uuid;

use crate::messages::StringMessage;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Client {
    id: Uuid,
    string_addr: Recipient<StringMessage>,
}

impl Client {
    pub fn new(id: Uuid, addr: Recipient<StringMessage>) -> Self {
        Self {
            id,
            string_addr: addr,
        }
    }

    pub fn addr(&self) -> &Recipient<StringMessage> {
        &self.string_addr
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
