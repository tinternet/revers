use reversi::templates;
use wasm_bindgen::prelude::*;

mod reversi;

#[wasm_bindgen]
pub struct Board {
    pub player: u64,
    pub enemy: u64,
    choices: Vec<reversi::EvaluatedPosition>,
}

#[wasm_bindgen]
pub struct FinalScore {
    pub player: usize,
    pub enemy: usize,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(inverse: bool) -> Board {
        let mut board = Board {
            player: templates::INIT_BLACK,
            enemy: templates::INIT_WHITE,
            choices: vec![],
        };

        if inverse {
            board.player = templates::INIT_WHITE;
            board.enemy = templates::INIT_BLACK;
            reversi::smart_pick(&mut board.enemy, &mut board.player);
        }

        board.find_choices();
        board
    }

    #[wasm_bindgen]
    pub fn get_choices(&self) -> Vec<Choice> {
        self.choices.clone().into_iter().map(|c| c.into()).collect()
    }

    #[wasm_bindgen]
    pub fn pick(&mut self, x: i32, y: i32) -> Option<FinalScore> {
        if let Some(choice) = self.choices.iter().find(|&c| c.pos.x == x && c.pos.y == y) {
            reversi::user_pick(&mut self.player, &mut self.enemy, choice);
            reversi::smart_pick(&mut self.enemy, &mut self.player);
            self.choices = reversi::find_choices(self.player, self.enemy, None);

            while self.choices.len() == 0 {
                if !reversi::smart_pick(&mut self.player, &mut self.enemy) {
                    let player = self.player.count_ones() as usize;
                    let enemy = self.enemy.count_ones() as usize;
                    return Some(FinalScore { player, enemy });
                }
                self.choices = reversi::find_choices(self.player, self.enemy, None);
            }
        }
        None
    }

    fn find_choices(&mut self) {
        self.choices = reversi::find_choices(self.player, self.enemy, None)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<reversi::Position> for Position {
    fn from(pos: reversi::Position) -> Self {
        Position {
            x: pos.x as i32,
            y: pos.y as i32,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Choice {
    pos: Position,
    taken: Vec<Position>,
}

impl From<reversi::EvaluatedPosition> for Choice {
    fn from(choice: reversi::EvaluatedPosition) -> Self {
        Choice {
            pos: choice.pos.into(),
            taken: choice.taken.iter().map(|&x| x.into()).collect(),
        }
    }
}

#[wasm_bindgen]
impl Choice {
    #[wasm_bindgen]
    pub fn get_taken(&self) -> js_sys::Array {
        self.taken.iter().map(|&x| JsValue::from(x)).collect()
    }

    #[wasm_bindgen]
    pub fn get_pos(&self) -> Position {
        self.pos
    }
}
