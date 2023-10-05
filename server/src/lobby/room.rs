use std::{collections::HashSet, sync::Arc};

use engine::{
    board::Board,
    piece::{position::Position, ChessPiece, Color, Type},
    result::OkMovement,
};

use crate::messages::result::ResultMessage;

use super::{client::Client, errors::RoomError, ClientId, RoomId};

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnMoveType {
    Movement(OkMovement),
    Promotion { to: ChessPiece, on: Position },
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnMove {
    pub turn_number: u32,
    pub piece: ChessPiece,
    pub client_id: ClientId,
    #[serde(rename = "type", flatten)]
    pub turn_move_type: TurnMoveType,
}

#[derive(Clone)]
pub struct Room {
    id: RoomId,
    clients: HashSet<Arc<Client>>,
    white: Option<ClientId>,
    black: Option<ClientId>,
    board: Board,
    turn_number: u32,
    moves: Vec<TurnMove>,
}

impl Room {
    pub fn new(id: RoomId) -> Self {
        Self {
            id,
            clients: HashSet::new(),
            white: None,
            black: None,
            board: Board::new(),
            turn_number: 1,
            moves: Vec::new(),
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

        let turn_num = self.turn_number;
        let turn = self.board.get_turn();

        let result = self.board.move_piece(from, to);

        let ok_move = match result {
            Ok(movement) => {
                let promotion = self.board.get_promotion_color();
                let check = self.board.get_check();
                let result = ResultMessage::movement(
                    self.id,
                    client_id,
                    movement,
                    promotion,
                    check,
                    self.turn_number,
                );

                self.send_room_result(result);
                movement
            }
            Err(e) => {
                let client = self.client(client_id)?;
                let err = ResultMessage::error(self.id, client_id, e.to_string());
                client.result_addr().do_send(err);
                return Ok(());
            }
        };

        let turn_move = TurnMove {
            turn_number: turn_num,
            piece: *self.board.get_piece_at(&to).unwrap(), //SAFE: We just moved this piece
            client_id,
            turn_move_type: TurnMoveType::Movement(ok_move),
        };
        self.moves.push(turn_move);

        if turn == Color::Black {
            self.turn_number += 1;
        }

        if let Some(winner) = self.board.get_winner() {
            let result = ResultMessage::winner(self.id, client_id, winner);
            self.send_room_result(result);
        }

        Ok(())
    }

    pub fn promote(&mut self, client_id: ClientId, piece: Type) -> Result<(), RoomError> {
        self.can_play(client_id)?;
        let color = self.get_color(client_id)?;
        let turn = self.board.get_turn();
        let turn_num = {
            if turn == Color::White {
                self.turn_number
            } else {
                self.turn_number - 1
            }
        };

        let piece = match piece {
            Type::Queen => ChessPiece::create_queen(color),
            Type::Rook => ChessPiece::create_rook(color),
            Type::Bishop => ChessPiece::create_bishop(color),
            Type::Knight => ChessPiece::create_knight(color),
            Type::Pawn => ChessPiece::create_pawn(color),
            Type::King => ChessPiece::create_king(color),
        };

        let result = self.board.promote(piece);

        let promotion = match result {
            Ok(promotion) => {
                let check = self.board.get_check();
                let result = ResultMessage::promotion(self.id, client_id, promotion, check);

                self.send_room_result(result);
                promotion
            }
            Err(e) => {
                let client = self.client(client_id)?;
                let err = ResultMessage::error(self.id, client_id, e.to_string());
                client.result_addr().do_send(err);
                return Ok(());
            }
        };

        let turn_move = TurnMove {
            turn_number: turn_num,
            piece: ChessPiece::create_pawn(color),
            client_id,
            turn_move_type: TurnMoveType::Promotion {
                to: piece,
                on: promotion.0,
            },
        };

        self.moves.push(turn_move);

        Ok(())
    }

    pub fn resign(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        self.can_play(client_id)?;

        self.board.resign();

        if let Some(winner) = self.board.get_winner() {
            let result = ResultMessage::winner(self.id, client_id, winner);
            self.send_room_result(result);
        }

        Ok(())
    }

    pub fn reset(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        self.can_play(client_id)?;

        if self.board.get_winner().is_none() {
            return Err(RoomError::GameNotOver);
        };

        self.board.reset();

        let result = ResultMessage::reset(self.id, client_id);

        self.send_room_result(result);

        Ok(())
    }

    pub fn enemy_id(&self, client_id: ClientId) -> Result<Option<ClientId>, RoomError> {
        let color = self.get_color(client_id)?;
        let enemy_id = match color {
            Color::White => self.black,
            Color::Black => self.white,
        };
        Ok(enemy_id)
    }

    pub fn get_color(&self, client_id: ClientId) -> Result<Color, RoomError> {
        if self.white == Some(client_id) {
            Ok(Color::White)
        } else if self.black == Some(client_id) {
            Ok(Color::Black)
        } else {
            Err(RoomError::ClientNotInRoom)
        }
    }

    pub fn pieces(&self) -> [[Option<ChessPiece>; 8]; 8] {
        *self.board.get_pieces()
    }

    pub fn moves(&self) -> &Vec<TurnMove> {
        &self.moves
    }

    pub fn id(&self) -> RoomId {
        self.id
    }

    pub fn check(&self) -> Option<Color> {
        self.board.get_check()
    }

    pub fn promotion(&self) -> Option<Color> {
        self.board.get_promotion_color()
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

    fn client(&self, client_id: ClientId) -> Result<&Arc<Client>, RoomError> {
        let client = self
            .clients
            .iter()
            .find(|client| client.id() == client_id)
            .ok_or(RoomError::ClientNotInRoom)?;
        Ok(client)
    }

    fn send_room_result(&self, msg: ResultMessage) {
        for client in &self.clients {
            client.result_addr().do_send(msg.clone());
        }
    }
}
