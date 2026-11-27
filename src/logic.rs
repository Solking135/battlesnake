use log::info;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Game};

pub fn info() -> Value {
    info!("INFO");
    json!({
        "apiversion": "1",
        "author": "Solking135",
        "color": "#7f00ff",
        "head": "evil",
        "tail": "hook",
    })
}

pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

const OFFSET: usize = 19;
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    let height: i32 = _board.height;
    let width: i32 = _board.width;
    let mut hazards = [0u8; 361];
    let snakes: &Vec<Battlesnake> = &_board.snakes;
    let snake_count: usize = snakes.len();
    let head: &Coord = &you.body[0];
    let neck: &Coord = &you.body[1];
    let food: &Vec<Coord> = &_board.food;
    for i in 0..snake_count {
        let body: &Vec<Coord> = &snakes[i].body;
        let body_length: usize = body.len();
        for j in 0..body_length {
            hazards[(body[j].x as usize) + (body[j].y as usize) * OFFSET] = 1;
        }
    }
    let pos: usize = (head.x as usize) + (head.y as usize) * OFFSET;
    moves_safe |= ((head.x > 0) && (hazards[pos - 1] == 0)) as u8;
    moves_safe |= (((head.x < width - 1) && (hazards[pos + 1] == 0)) as u8) << 1u8;
    moves_safe |= (((head.y > 0) && (hazards[pos - OFFSET] == 0)) as u8) << 2u8;
    moves_safe |= (((head.y < height - 1) && (hazards[pos + OFFSET] == 0)) as u8) << 3u8;
    info!("MOVE {}: {}", turn, chosen);
    json!({ "move": chosen })
}