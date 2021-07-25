use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Board {
    state: [[usize; 8]; 8],
    next: usize,
}

impl Board {
    pub fn new() -> Board {
        let state = [
            [0; 8],
            [0; 8],
            [0, 0, 0, 0, 3, 0, 0, 0],
            [0, 0, 0, 1, 2, 3, 0, 0],
            [0, 0, 3, 2, 1, 0, 0, 0],
            [0, 0, 0, 3, 0, 0, 0, 0],
            [0; 8],
            [0; 8]
        ] as [[usize; 8]; 8];
        Board { state, next: 1 }
    }
}