use log::info;
use rand::Rng;
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

const SELECT: [[u8; 4]; 16] = [
    [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 1, 0, 0],
    [2, 2, 2, 2], [0, 2, 0, 0], [1, 2, 0, 0], [0, 1, 2, 0],
    [3, 3, 3, 3], [0, 3, 0, 0], [1, 3, 0, 0], [0, 1, 3, 0],
    [2, 3, 0, 0], [0, 2, 3, 0], [1, 2, 3, 0], [0, 1, 2, 3],
];
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    let mut moves_safe: u8 = 0;
    let mut hazards: u8 = 0;
    let height = _board.height;
    let width = _board.width;
    let food = &_board.food;
    let snakes = &_board.snakes;
    let snake_count = snakes.len();
    let head = &you.body[0];
    let neck = &you.body[1];
    moves_safe |= ((head.x != 0) && (head.x <= neck.x)) as u8;
    moves_safe |= ((head.x != width - 1) && (head.x >= neck.x)) as u8 << 1u8;
    moves_safe |= ((head.y != 0) && (head.y <= neck.y)) as u8 << 2u8;
    moves_safe |= ((head.y != height - 1) && (head.y >= neck.y)) as u8 << 3u8;
    for i in 0..snake_count {
        let body = &snakes[i].body;
        let body_length = body.len();
        for j in 0..body_length {
            hazards |= ((head.y == body[j].y) && (head.x == body[j].x + 1)) as u8;
            hazards |= ((head.y == body[j].y) && (head.x == body[j].x - 1)) as u8 << 1u8;
            hazards |= ((head.x == body[j].x) && (head.y == body[j].y + 1)) as u8 << 2u8;
            hazards |= ((head.x == body[j].x) && (head.y == body[j].y - 1)) as u8 << 3u8;
        }
    }
    moves_safe &= ~hazards;
    // REVIEW
    let mask = (moves_safe & 0x0f);
    let count = mask.count_ones();
    let rank = rand::rng().random_range(0..count) as usize;
    let direction = SELECT[mask as usize][rank] as usize;
    let chosen = ["left", "right", "down", "up"][direction];
    //
    info!("MOVE {}: {}", turn, chosen);
    json!({ "move": chosen })
}