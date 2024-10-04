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
                new_board[i][j] = self.board[i][j].tick(self.get_kernel(i, j));
            }
        }
        new_board
    }

    fn get_kernel(&self, i: usize, j: usize) -> Vec<Option<&Cell>> {
        let (i, j) = (i as isize, j as isize);
        let mut kernel = vec![];
        for u in 0..3 {
            for v in 0..3 {
                kernel.push(self.get_tile(i - 1 + u, j - 1 + v))
            }
        }
        let mut found = false;
        for n in 0..kernel.len() {
            if !found {
                if kernel[n] == self.get_tile(i, j) {kernel.remove(n); found = true}
            }
        }
        kernel
    }

    fn get_tile(&self, w: isize, h: isize) -> Option<&Cell> {
        if w < 0 || h < 0 || w >= self.width as isize || h  >= self.height as isize {
            return None
        } else {
            let w: usize = w.try_into().unwrap();
            let h: usize = h.try_into().unwrap();
            let c = self.board.get(w);
            match c {
                Some(c) => return c.get(h),
                None => return None
            }
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
                    if cell == &&Cell::ALIVE {1}
                    else {0}
                },
                None => 0
            }
        }).sum();

        if self == &Cell::ALIVE {
            if k > 2 {Cell::DEAD}
            else if k == 2 || k == 3 {Cell::ALIVE}
            else {Cell::DEAD}
        } else {
            if k == 3 {Cell::ALIVE}
            else {Cell::DEAD}
        }
    }
}
