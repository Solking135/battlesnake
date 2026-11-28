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

const Y_OFFSET: usize = 21;
const BOARD_SIZE: usize = Y_OFFSET * Y_OFFSET;
const MOVES: [isize; 4] = [-1, 1, -21, 21];
const DIRECTIONS: [&str; 4] = ["left", "right", "down", "up"];
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    let mut hazards = [1u8; BOARD_SIZE];
    for y in 1..=(_board.height) as usize {
        for x in 1..=(_board.width) as usize {
            hazards[x + y * Y_OFFSET] = 0;
        }
    }
    let snakes: &Vec<Battlesnake> = &_board.snakes;
    let snake_count: usize = snakes.len();
    for i in 0..snake_count {
        let body = &snakes[i].body;
        let body_length: usize = body.len();
        for j in 0..body_length {
            hazards[(body[j].x as usize + 1) + (body[j].y as usize + 1) * Y_OFFSET] = 1;
        }
    }
    let pos: usize = (you.head.x + 1 + (you.head.y + 1) * Y_OFFSET as i32) as usize;
    let mut score_best: i32 = i32::MIN;
    let mut direction_best: usize = 0;
    let food = &_board.food;
    for i in 0..4 {
        let next: usize = (pos as isize + MOVES[i]) as usize;
        if hazards[next] == 0 {
            let mobility: i32 = (hazards[next - 1] == 0) as i32 + (hazards[next + 1] == 0) as i32 + (hazards[next - Y_OFFSET] == 0) as i32 + (hazards[next + Y_OFFSET] == 0) as i32;
            let mut food_dist: i32 = i32::MAX;
            let x: i32 = (next % Y_OFFSET) as i32 - 1;
            let y: i32 = (next / Y_OFFSET) as i32 - 1;
            for f in food {
                let dist = (x - f.x).abs() + (y - f.y).abs();
                food_dist = food_dist.min(dist);
            }
            food_dist *= (food_dist != i32::MAX) as i32;
            let score: i32 = mobility * 3 - (food_dist << 1);
            let mask: usize = 0usize.wrapping_sub((score > score_best) as usize);
            score_best = (score & mask as i32) | (score_best & !mask as i32);
            direction_best = (i & mask) | (direction_best & !mask);
        }
    }
    let chosen: &str = DIRECTIONS[direction_best];
    info!("MOVE {}: {}", turn, chosen);
    json!({ "move": chosen })
}