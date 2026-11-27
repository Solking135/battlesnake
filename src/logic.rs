use log::info;
use rand::seq::IndexedRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

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

pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

const SELECT: [[u8; 4]; 16] = [
    [0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 1, 0, 0],
    [2, 2, 2, 2], [0, 2, 0, 0], [1, 2, 0, 0], [0, 1, 2, 0],
    [3, 3, 3, 3], [0, 3, 0, 0], [1, 3, 0, 0], [0, 1, 3, 0],
    [2, 3, 0, 0], [0, 2, 3, 0], [1, 2, 3, 0], [0, 1, 2, 3],
];
pub fn get_move(_game: &Game, turn: &i32, _board: &Board, you: &Battlesnake) -> Value {
    let mut moves_safe: i32 = 0;
    let mut hazards: i32 = 0;
    let height: i32 = _board.height;
    let width: i32 = _board.width;
    let food: Vec<Coord> = &_board.food;
    let snakes: Vec<Battlesnake> = &_board.snakes;
    let snake_count: i32 = snakes.len();
    let head: Coord = &you.body[0];
    let neck: Coord = &you.body[1];
    moves_safe |= (head.x != 0) & (head.x <= neck.x);
    moves_safe |= ((head.x != width - 1) & (head.x >= neck.x)) << 1;
    moves_safe |= ((head.y != 0) & (head.y <= neck.y)) << 2;
    moves_safe |= ((head.y != height - 1) & (head.y >= neck.y)) << 3;
    for i in 0..snake_count {
        let body = snakes[i];
        let body_length = body.len();
        for j in 0..body_length {
            hazards |= (head.y == body[j].y) & (head.x == body[j].x + 1);
            hazards |= ((head.y == body[j].y) & (head.x == body[j].x - 1)) << 1;
            hazards |= ((head.x == body[j].x) & (head.y == body[j].y + 1)) << 2;
            hazards |= ((head.x == body[j].x) & (head.y == body[j].y - 1)) << 3;
        }
    }
    moves_safe &= ~hazards;
    // REVIEW
    let mask = (moves_safe as u8 & 0x0f) | u8::from(moves_safe == 0);
    let count = mask.count_ones();
    let rank = rand::rng().random_range(0..count) as usize;
    let direction = SELECT[mask as usize][rank] as usize;
    let chosen = ["left", "right", "down", "up"][direction];
    //
    info!("MOVE {}: {}", turn, chosen);
    json!({ "move": chosen })
}