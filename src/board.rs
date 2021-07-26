use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    state: [[usize; 8]; 8],
    next: usize,
    new: Option<(usize, usize)>
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
        Board { state, next: 1, new: None }
    }

    pub fn put(&mut self, row_pos: usize, col_pos: usize) -> Result<(),String> {
        if !((0..8).contains(&row_pos) && (0..8).contains(&col_pos)) {
            return Err(String::from("argument(s) out of bound"));
        }
        if self.check(row_pos, col_pos) == 0 {
            return Err(String::from("no reverses"));
        }
        self.reverse(row_pos, col_pos);
        self.next = 3 - self.next;
        for i in 0..8 {
            for j in 0..8 {
                if self.state[i][j] > 2 {
                    self.state[i][j] = 0;
                }
                if self.state[i][j] == 0 && self.check(i, j) > 0 {
                    self.state[i][j] = self.next + 2;
                }
            }
        }
        Ok(())
    }

    fn check(&self, row_pos: usize, col_pos: usize) -> usize {
        if let 1 | 2 = self.state[row_pos][col_pos] {
            return 0;
        }
        let cnt = [(2,1),(2,2),(1,2),(0,2),(0,1),(0,0),(1,0),(2,0)].iter().map(|(di, dj)| {
            let mut cnt: usize = 0;
            let mut i = row_pos;
            let mut j = col_pos;
            loop {
                if i + di < 1 || i + di > 8 || j + dj < 1 || j + dj > 8 {
                    return 0;
                }
                i += di;
                i -= 1;
                j += dj;
                j -= 1;
                if let 0 | 3 | 4 = self.state[i][j] {
                    return 0;
                }
                if self.state[i][j] == self.next {
                    break;
                }
                cnt += 1;
            }
            cnt
        }).sum();
        cnt
    }

    fn reverse(&mut self, row_pos: usize, col_pos: usize) {
        self.state[row_pos][col_pos] = self.next;
        self.new = Some((row_pos, col_pos));
        [(2,1),(2,2),(1,2),(0,2),(0,1),(0,0),(1,0),(2,0)].iter().for_each(|(di, dj)| {
            let mut i = row_pos;
            let mut j = col_pos;
            loop {
                if i + di < 1 || i + di > 8 || j + dj < 1 || j + dj > 8 {
                    return;
                }
                i += di; 
                i -= 1;
                j += dj; 
                j -= 1;
                if let 0 | 3 | 4 = self.state[i][j] {
                    return;
                }
                if self.state[i][j] == self.next {
                    break;
                }
            }
            loop {
                if i + di < 1 || i + di > 8 || j + dj < 1 || j + dj > 8 {
                    return;
                }
                i += 1; 
                i -= di;
                j += 1; 
                j -= dj;
                if self.state[i][j] == self.next {
                    return;
                }
                self.state[i][j] = self.next;
            }
        });
    }
}
