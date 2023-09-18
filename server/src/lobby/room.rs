use std::{collections::HashSet, sync::Arc};

use engine::{
    board::Board,
    piece::{position::Position, ChessPiece, Color, Type},
};

use crate::messages::{
    success::{SuccessMessage, SuccessResult},
    ErrorMessage,
};

use super::{client::Client, errors::RoomError, ClientId, RoomId};

#[derive(PartialEq, Eq, Clone)]
pub struct Room {
    id: RoomId,
    clients: HashSet<Arc<Client>>,
    white: Option<ClientId>,
    black: Option<ClientId>,
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
        let client_id = client.id();
        if self.clients.len() >= 2 {
            return Err(RoomError::RoomFull);
        }
        if self.clients.contains(&client) {
            return Err(RoomError::ClientAlreadyInRoom);
        }
        self.clients.insert(client);
        if self.white.is_none() {
            self.white = Some(client_id);
            color = Color::White;
        } else {
            self.black = Some(client_id);
            color = Color::Black;
        }
        Ok(color)
    }

    pub fn remove_client(&mut self, client: &Arc<Client>) -> Result<(), RoomError> {
        let client_id = client.id();
        if !self.clients.contains(client) {
            return Err(RoomError::ClientNotInRoom);
        }
        self.clients.remove(client);

        if self.white == Some(client_id) {
            self.white = None;
        } else {
            self.black = None;
        }
        Ok(())
    }

    pub fn make_move(
        &mut self,
        client_id: ClientId,
        from: Position,
        to: Position,
    ) -> Result<(), RoomError> {
        self.can_play(client_id)?;

        let result = self.board.move_piece(from, to);

        match result {
            Ok(movement) => {
                let result = SuccessMessage {
                    client_id,
                    room_id: self.id,
                    result: SuccessResult::Movement(movement),
                };

                self.send_room_success(result);
            }
            Err(e) => {
                let client = self.client(client_id)?;
                client.error_addr().do_send(ErrorMessage {
                    client_id,
                    error: e.to_string(),
                    room_id: self.id,
                });
            }
        }

        Ok(())
    }

    pub fn promote(&mut self, client_id: ClientId, piece: Type) -> Result<(), RoomError> {
        self.can_play(client_id)?;
        let color = self.get_color(client_id)?;

        let piece = match piece {
            Type::Queen => ChessPiece::create_queen(color),
            Type::Rook => ChessPiece::create_rook(color),
            Type::Bishop => ChessPiece::create_bishop(color),
            Type::Knight => ChessPiece::create_knight(color),
            Type::Pawn => ChessPiece::create_pawn(color),
            Type::King => ChessPiece::create_king(color),
        };

        let result = self.board.promote(piece);

        match result {
            Ok(promotion) => {
                let result = SuccessMessage {
                    client_id,
                    room_id: self.id,
                    result: SuccessResult::Promotion(promotion),
                };

                self.send_room_success(result);
            }
            Err(e) => {
                let client = self.client(client_id)?;
                client.error_addr().do_send(ErrorMessage {
                    client_id,
                    error: e.to_string(),
                    room_id: self.id,
                });
            }
        }

        Ok(())
    }

    pub fn resign(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        self.can_play(client_id)?;

        let _result = self.board.resign();
        Ok(())
    }

    fn can_play(&self, client_id: ClientId) -> Result<(), RoomError> {
        let color = self.get_color(client_id)?;
        let turn = self.board.get_turn();
        if self.white.is_none() || self.black.is_none() {
            return Err(RoomError::NotEnoughPlayers);
        }
        if color != turn {
            return Err(RoomError::NotYourTurn);
        }

        Ok(())
    }

    fn get_color(&self, client_id: ClientId) -> Result<Color, RoomError> {
        if self.white == Some(client_id) {
            Ok(Color::White)
        } else if self.black == Some(client_id) {
            Ok(Color::Black)
        } else {
            Err(RoomError::ClientNotInRoom)
        }
    }

    fn client(&self, client_id: ClientId) -> Result<&Arc<Client>, RoomError> {
        let client = self
            .clients
            .iter()
            .find(|client| client.id() == client_id)
            .ok_or(RoomError::ClientNotInRoom)?;
        Ok(client)
    }

    fn send_room_success(&self, msg: SuccessMessage) {
        for client in &self.clients {
            client.success_addr().do_send(msg.clone());
        }
    }
}
