fn main() {
    let mut b = Board::new(4, 4);
    b.board = b.update();
}

struct Board {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<Cell>>
}

impl Board {
    pub fn new(w: usize, h: usize) -> Board {
        Board {
            width: w,
            height: h,
            board: vec![vec![Cell::DEAD; h];w]
        }
    }

    pub fn update(&self) -> Vec<Vec<Cell>> {
        let mut new_board = vec![vec![Cell::DEAD; self.height]; self.width];
        for i in 0..self.width {
            for j in 0..self.height {
                let mut kernel: Vec<Option<&Cell>> = vec![];
                for u in (i - 1) .. (i + 1) {
                    for v in (i - 1) .. (i + 1) {
                        if u != v {kernel.push(self.get(i, j))}
                    }
                } 
                new_board[i][j] = self.board[i][j].tick(kernel);
            }
        }
        new_board
    }

    fn get(&self, w: usize, h: usize) -> Option<&Cell> {
        let c = self.board.get(w);
        match c {
            Some(c) => return c.get(h),
            None => return None
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = "".to_owned();
        for i in &self.board {
            let mut s = "|".to_owned();
            for j in i {
                match j {
                    Cell::DEAD => s += " [ ] |",
                    Cell::ALIVE => s += " [X] |",
                }
            }
            out += &(s + "\n");
        }
        write!(f, "{out}")
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    ALIVE,
    DEAD,
}

impl Cell {
    pub fn tick(&self, kernel: Vec<Option<&Cell>>) -> Cell {
        let k: u8 = kernel.iter().map(|c| {
            match c {
                Some(cell) => {
                    if **cell == Cell::ALIVE {1}
                    else {0}
                },
                None => 0
            }
        }).sum();

        if *self == Cell::ALIVE {
            if k > 2 {Cell::DEAD}
            else if k == 2 || k == 3 {Cell::ALIVE}
            else {Cell::DEAD}
        } else {
            if k == 3 {Cell::ALIVE}
            else {Cell::DEAD}
        }
    }
}
