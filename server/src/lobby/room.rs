use std::{collections::HashSet, sync::Arc};

use engine::{board::Board, piece::Color};

use super::{client::Client, errors::RoomError, RoomId};

#[derive(PartialEq, Eq, Clone)]
pub struct Room {
    id: RoomId,
    clients: HashSet<Arc<Client>>,
    white: Option<Arc<Client>>,
    black: Option<Arc<Client>>,
    board: Board,
}

impl Room {
    pub fn new(id: RoomId) -> Self {
        Self {
            id,
            clients: HashSet::new(),
            white: None,
            black: None,
            board: Board::new(),
        }
    }

    pub fn clients(&self) -> &HashSet<Arc<Client>> {
        &self.clients
    }

    pub fn add_client(&mut self, client: Arc<Client>) -> Result<Color, RoomError> {
        let color: Color;
        if self.clients.len() >= 2 {
            return Err(RoomError::RoomFull);
        }
        if self.clients.contains(&client) {
            return Err(RoomError::ClientAlreadyInRoom);
        }
        self.clients.insert(client.clone());
        if self.white.is_none() {
            self.white = Some(client);
            color = Color::White;
        } else {
            self.black = Some(client);
            color = Color::Black;
        }
        Ok(color)
    }

    pub fn remove_client(&mut self, client: &Arc<Client>) -> Result<(), RoomError> {
        if !self.clients.contains(client) {
            return Err(RoomError::ClientNotInRoom);
        }
        self.clients.remove(client);

        if self.white.as_ref() == Some(client) {
            self.white = None;
        } else {
            self.black = None;
        }
        Ok(())
    }
}
