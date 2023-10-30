use actix::{Actor, AsyncContext, Recipient};
use engine::{
    board::{Board, GameState},
    piece::{position::Position, ChessPiece, Color, Type},
    result::OkMovement,
};

use crate::messages::result::{ConnectionType, ResultMessage};

use self::message::RoomMessage;

use super::{client::Client, errors::RoomError, ClientId, RoomId};

use serde::Serialize;

mod actor;
pub mod message;

//10 minutes in milliseconds
const MAX_TIME: u32 = 600_000;

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
    white: Option<Client>,
    black: Option<Client>,
    board: Board,
    turn_number: u32,
    moves: Vec<TurnMove>,
    lobby: Recipient<RoomMessage>,
    black_time: u32,
    white_time: u32,
    black_timer_ticking: bool,
    white_timer_ticking: bool,
}

impl Room {
    pub fn new(id: RoomId, lobby: Recipient<RoomMessage>) -> Self {
        Self {
            id,
            white: None,
            black: None,
            board: Board::new(),
            turn_number: 1,
            moves: Vec::new(),
            lobby,
            black_time: MAX_TIME,
            white_time: MAX_TIME,
            black_timer_ticking: false,
            white_timer_ticking: false,
        }
    }

    /// Returns an error if the client is already in the room
    /// Returns true if the room is full
    pub fn add_client(&mut self, client: Client) -> Result<bool, RoomError> {
        let client_id = client.id();
        let mut inserted = false;
        if self.white.is_some() && self.black.is_some() {
            return Err(RoomError::RoomFull);
        }

        if self.client(client_id).is_some() {
            return Err(RoomError::ClientAlreadyInRoom);
        }

        let (white_client, white_msg_type) = match &self.white {
            Some(white_client) => (white_client, ConnectionType::EnemyClient),
            None => {
                self.white = Some(client.clone());
                inserted = true;
                (&client, ConnectionType::SelfClient)
            }
        };

        let black_message_opt = match (&self.black, inserted) {
            (Some(black_client), true) => Some((black_client, ConnectionType::EnemyClient)),
            (None, false) => {
                self.black = Some(client.clone());

                Some((&client, ConnectionType::SelfClient))
            }
            _ => None,
        };

        let white_msg = ResultMessage::connect(white_client.id(), white_msg_type, &self);
        white_client.result_addr().do_send(white_msg);

        if let Some((black_client, black_message)) = black_message_opt {
            let black_msg = ResultMessage::connect(black_client.id(), black_message, &self);
            black_client.result_addr().do_send(black_msg);
        }

        let full = self.white.is_some() && self.black.is_some();

        Ok(full)
    }

    /// Returns an error if the client is not in the room
    /// Returns true if the room is empty
    pub fn remove_client(&mut self, client_id: ClientId) -> Result<bool, RoomError> {
        let enemy = self.enemy(client_id);

        match enemy {
            Some(enemy) => {
                let con_msg = ResultMessage::disconnect(self.id, client_id);
                enemy.result_addr().do_send(con_msg);
            }
            None => {}
        }

        match &self.white {
            Some(white_client) => {
                if white_client.id() == client_id {
                    self.white = None;
                }
            }
            None => {}
        };

        match &self.black {
            Some(black_client) => {
                if black_client.id() == client_id {
                    self.black = None;
                }
            }
            None => {}
        };

        let empty = self.white.is_none() && self.black.is_none();

        return Ok(empty);
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
                let err = ResultMessage::error(self.id, client_id, e.to_string());
                let client = self.client(client_id).ok_or(RoomError::ClientNotInRoom)?; //TODO: Handle this better
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

        let promotion = self.board.get_promotion_color();

        match promotion {
            Some(_) => {}
            None => {
                self.change_turn();
            }
        }

        self.moves.push(turn_move);

        if turn == Color::Black {
            self.turn_number += 1;
        }

        match self.board.get_state() {
            GameState::Draw => {
                /*
                let result = ResultMessage::draw(self.id, client_id);
                self.send_room_result(result);
                self.stop_game();
                 */
            }
            GameState::Winner(winner) => {
                let result = ResultMessage::winner(self.id, client_id, winner);
                self.send_room_result(result);
                self.stop_game();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn promote(&mut self, client_id: ClientId, piece: Type) -> Result<(), RoomError> {
        self.can_play(client_id)?;
        let color = self
            .client_color(client_id)
            .ok_or(RoomError::ClientNotInRoom)?;
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
                let err = ResultMessage::error(self.id, client_id, e.to_string());
                let client = self.client(client_id).ok_or(RoomError::ClientNotInRoom)?;
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

        self.change_turn();

        self.moves.push(turn_move);

        Ok(())
    }

    //REFACTOR
    pub fn resign(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        self.can_play(client_id)?;

        let winner = self.board.resign();

        let result = ResultMessage::winner(self.id, client_id, winner);
        self.send_room_result(result);
        self.stop_game();

        Ok(())
    }

    //REFACTOR
    pub fn reset(&mut self, client_id: ClientId) -> Result<(), RoomError> {
        self.can_play(client_id)?;

        match self.board.get_state() {
            GameState::Draw => {}
            GameState::Winner(_) => {}
            _ => return Err(RoomError::GameNotOver),
        };

        self.board.reset();

        let result = ResultMessage::reset(self.id, client_id);

        self.send_room_result(result);

        Ok(())
    }

    pub fn client_color(&self, client_id: ClientId) -> Option<Color> {
        match &self.white {
            Some(white_client) => {
                if white_client.id() == client_id {
                    return Some(Color::White);
                }
            }
            None => {}
        };

        match &self.black {
            Some(black_client) => {
                if black_client.id() == client_id {
                    return Some(Color::Black);
                }
            }
            None => {}
        };

        None
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

    pub fn timers(&self) -> (u32, u32) {
        (self.white_time, self.black_time)
    }

    pub fn client(&self, client_id: ClientId) -> Option<&Client> {
        match &self.white {
            Some(white_client) => {
                if white_client.id() == client_id {
                    return Some(white_client);
                }
            }
            None => {}
        }

        match &self.black {
            Some(black_client) => {
                if black_client.id() == client_id {
                    return Some(black_client);
                } else {
                    return None;
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn enemy(&self, client_id: ClientId) -> Option<&Client> {
        self.client(client_id)?;
        let enemy = match &self.white {
            Some(white_client) => {
                if white_client.id() == client_id {
                    self.black.as_ref()
                } else {
                    Some(white_client)
                }
            }
            None => None,
        };

        enemy
    }

    fn can_play(&self, client_id: ClientId) -> Result<(), RoomError> {
        let color = self
            .client_color(client_id)
            .ok_or(RoomError::ClientNotInRoom)?;
        let turn = self.board.get_turn();
        if self.white.is_none() || self.black.is_none() {
            return Err(RoomError::NotEnoughPlayers);
        }
        if color != turn {
            return Err(RoomError::NotYourTurn);
        }

        Ok(())
    }

    fn send_room_result(&self, msg: ResultMessage) {
        match &self.white {
            Some(white_client) => {
                white_client.result_addr().do_send(msg.clone());
            }
            None => {}
        };

        match &self.black {
            Some(black_client) => {
                black_client.result_addr().do_send(msg);
            }
            None => {}
        };
    }

    fn start_timer(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(std::time::Duration::from_millis(1), |act, _| {
            if act.white.is_none() || act.black.is_none() {
                return;
            }

            //>0 to avoid underflow
            if act.black_timer_ticking && act.black_time > 0 {
                act.black_time -= 1;
                let black = &act.black;
                let white = &act.white;
                let black_id = black.as_ref().map(|c| c.id());
                let timer = ResultMessage::timer(act.id, black_id, act.black_time, Color::Black);
                match white {
                    Some(white) => {
                        white.result_addr().do_send(timer.clone());
                    }
                    None => {}
                }
                match black {
                    Some(black) => {
                        black.result_addr().do_send(timer);
                    }
                    None => {}
                }
            }

            //>0 to avoid underflow
            if act.white_timer_ticking && act.white_time > 0 {
                act.white_time -= 1;
                let black = &act.black;
                let white = &act.white;
                let white_id = white.as_ref().map(|c| c.id());
                let timer = ResultMessage::timer(act.id, white_id, act.white_time, Color::White);
                match white {
                    Some(white) => {
                        white.result_addr().do_send(timer.clone());
                    }
                    None => {}
                }
                match black {
                    Some(black) => {
                        black.result_addr().do_send(timer);
                    }
                    None => {}
                }
            }
        });
    }

    fn start_game(&mut self) {
        //not enough players
        if self.black.is_none() || self.white.is_none() {
            return;
        }

        //game already started
        if self.black_timer_ticking || self.white_timer_ticking {
            return;
        }

        self.black_timer_ticking = false;
        self.white_timer_ticking = true;
    }

    fn change_turn(&mut self) {
        self.black_timer_ticking = !self.black_timer_ticking;
        self.white_timer_ticking = !self.white_timer_ticking;
    }

    fn stop_game(&mut self) {
        self.black_timer_ticking = false;
        self.white_timer_ticking = false;
    }
}
