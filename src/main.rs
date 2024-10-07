use std::fmt;
use std::{thread, time};
use std::io::{stdout, Write};
use crossterm::{QueueableCommand, cursor};
use rand::Rng;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let input = Board::get_input();
    let mut b = Board::new(input.3, input.2);

    for i in input.1 {
        if i.2 == "g".to_string() {b.make_glider(i.1, i.0)}
        if i.2 == "p".to_string() {b.make_pulsar(i.1, i.0)}
        if i.2 == "c".to_string() {b.make_copperhead(i.1, i.0)}
        if i.2 == "t".to_string() {b.make_tile(i.1, i.0)}
        if i.2 == "lh".to_string() {b.make_line(i.1, i.0, false)}
        if i.2 == "lv".to_string() {b.make_line(i.1, i.0, true)}
        if i.2 == "b".to_string() {b.make_bottle(i.1, i.0)}
        if i.2 == "r".to_string() {b.rand()}
    }

    let mut stdout = stdout();
    loop {
        let _ = stdout.flush();
        let _ = stdout.queue(cursor::MoveTo(0, 0));
        println!("{b}");
        b.board = b.update();
        thread::sleep(time::Duration::from_secs_f32(input.0));

    }

    #[allow(unreachable_code)]
    Ok(())
}

struct Board {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Board {
        Board {
            width: x,
            height: y,
            board: vec![vec![Cell::DEAD; y];x],
        }
    }

    pub fn get_input() -> (f32, Vec<(usize, usize, String)>, usize, usize) {
        let mut rl = DefaultEditor::new().unwrap();
        let mut timestep = 0.0;
        let mut opts = vec![];
        let mut xmax: usize = 0;
        let mut ymax: usize = 0;

        let input = rl.readline(">>Timestep: ");
        match input {
            Ok(n) => timestep = n.trim().parse().unwrap_or(0.25),
            Err(rustyline::error::ReadlineError::Eof) => return (timestep, opts, xmax, ymax),
            _ => ()
        }

        let input = rl.readline(">>X maximum: ");
        match input {
            Ok(n) => xmax = n.trim().parse().unwrap_or(10),
            Err(rustyline::error::ReadlineError::Eof) => return (timestep, opts, xmax, ymax),
            _ => ()
        }

        let input = rl.readline(">>Y maximum: ");
        match input {
            Ok(n) => ymax = n.trim().parse().unwrap_or(10),
            Err(rustyline::error::ReadlineError::Eof) => return (timestep, opts, xmax, ymax),
            _ => ()
        }

        loop {
            let mut vals = (0, 0, "".to_string());
            let object_type = rl.readline(">>Type [g, p, c, t, lh, lv, b, r]: ");
            match object_type {
                Ok(s) => vals.2 = s,
                Err(rustyline::error::ReadlineError::Eof) => return (timestep, opts, xmax, ymax),
                _ => ()
            }
            let x_loc = rl.readline(">>X location: ");
            match x_loc {
                Ok(n) => vals.0 = n.trim().parse().unwrap_or(0),
                Err(rustyline::error::ReadlineError::Eof) => return (timestep, opts, xmax, ymax),
                _ => ()
            }
            let y_loc = rl.readline(">>Y location: ");
            match y_loc {
                Ok(n) => vals.1 =  n.trim().parse().unwrap_or(0),
                Err(rustyline::error::ReadlineError::Eof) => return (timestep, opts, xmax, ymax),
                _ => ()
            }
            opts.push(vals)
        }
    }

    pub fn make_copperhead(&mut self, x: usize, y: usize) {
        self.board[0 + x][3 + y] = Cell::ALIVE;
        self.board[0 + x][4 + y] = Cell::ALIVE;
        self.board[1 + x][3 + y] = Cell::ALIVE;
        self.board[1 + x][4 + y] = Cell::ALIVE;

        self.board[3 + x][2 + y] = Cell::ALIVE;
        self.board[3 + x][3 + y] = Cell::ALIVE;
        self.board[3 + x][4 + y] = Cell::ALIVE;
        self.board[3 + x][5 + y] = Cell::ALIVE;


        self.board[4 + x][1 + y] = Cell::ALIVE;
        self.board[4 + x][2 + y] = Cell::ALIVE;
        self.board[4 + x][5 + y] = Cell::ALIVE;
        self.board[4 + x][6 + y] = Cell::ALIVE;


        self.board[5 + x][0 + y] = Cell::ALIVE;
        self.board[5 + x][7 + y] = Cell::ALIVE;


        self.board[7 + x][0 + y] = Cell::ALIVE;
        self.board[7 + x][7 + y] = Cell::ALIVE;
        self.board[8 + x][0 + y] = Cell::ALIVE;
        self.board[8 + x][7 + y] = Cell::ALIVE;


        self.board[8 + x][2 + y] = Cell::ALIVE;
        self.board[8 + x][5 + y] = Cell::ALIVE;

        self.board[9 + x][3 + y] = Cell::ALIVE;
        self.board[9 + x][4 + y] = Cell::ALIVE;
        self.board[10 + x][3 + y] = Cell::ALIVE;
        self.board[10 + x][4 + y] = Cell::ALIVE;

        self.board[11 + x][1 + y] = Cell::ALIVE;
        self.board[11 + x][2 + y] = Cell::ALIVE;
        self.board[11 + x][5 + y] = Cell::ALIVE;
        self.board[11 + x][6 + y] = Cell::ALIVE;
    }

    pub fn make_pulsar(&mut self, x: usize, y: usize) {
        self.board[0 + x][2 + y] = Cell::ALIVE;
        self.board[0 + x][3 + y] = Cell::ALIVE;
        self.board[0 + x][4 + y] = Cell::ALIVE;

        self.board[0 + x][8 + y] = Cell::ALIVE;
        self.board[0 + x][9 + y] = Cell::ALIVE;
        self.board[0 + x][10 + y] = Cell::ALIVE;

        self.board[2 + x][0 + y] = Cell::ALIVE;
        self.board[2 + x][5 + y] = Cell::ALIVE;
        self.board[2 + x][7 + y] = Cell::ALIVE;
        self.board[2 + x][12 + y] = Cell::ALIVE;

        self.board[3 + x][0 + y] = Cell::ALIVE;
        self.board[3 + x][5 + y] = Cell::ALIVE;
        self.board[3 + x][7 + y] = Cell::ALIVE;
        self.board[3 + x][12 + y] = Cell::ALIVE;

        self.board[4 + x][0 + y] = Cell::ALIVE;
        self.board[4 + x][5 + y] = Cell::ALIVE;
        self.board[4 + x][7 + y] = Cell::ALIVE;
        self.board[4 + x][12 + y] = Cell::ALIVE;

        self.board[5 + x][2 + y] = Cell::ALIVE;
        self.board[5 + x][3 + y] = Cell::ALIVE;
        self.board[5 + x][4 + y] = Cell::ALIVE;

        self.board[5 + x][8 + y] = Cell::ALIVE;
        self.board[5 + x][9 + y] = Cell::ALIVE;
        self.board[5 + x][10 + y] = Cell::ALIVE;

        self.board[7 + x][2 + y] = Cell::ALIVE;
        self.board[7 + x][3 + y] = Cell::ALIVE;
        self.board[7 + x][4 + y] = Cell::ALIVE;

        self.board[7 + x][8 + y] = Cell::ALIVE;
        self.board[7 + x][9 + y] = Cell::ALIVE;
        self.board[7 + x][10 + y] = Cell::ALIVE;

        self.board[8 + x][0 + y] = Cell::ALIVE;
        self.board[8 + x][5 + y] = Cell::ALIVE;
        self.board[8 + x][7 + y] = Cell::ALIVE;
        self.board[8 + x][12 + y] = Cell::ALIVE;

        self.board[9 + x][0 + y] = Cell::ALIVE;
        self.board[9 + x][5 + y] = Cell::ALIVE;
        self.board[9 + x][7 + y] = Cell::ALIVE;
        self.board[9 + x][12 + y] = Cell::ALIVE;

        self.board[10 + x][0 + y] = Cell::ALIVE;
        self.board[10 + x][5 + y] = Cell::ALIVE;
        self.board[10 + x][7 + y] = Cell::ALIVE;
        self.board[10 + x][12 + y] = Cell::ALIVE;

        self.board[12 + x][2 + y] = Cell::ALIVE;
        self.board[12 + x][3 + y] = Cell::ALIVE;
        self.board[12 + x][4 + y] = Cell::ALIVE;

        self.board[12 + x][8 + y] = Cell::ALIVE;
        self.board[12 + x][9 + y] = Cell::ALIVE;
        self.board[12 + x][10 + y] = Cell::ALIVE;
    }

    pub fn make_glider(&mut self, x: usize, y: usize) {
        self.board[3 + x][0 + y] = Cell::ALIVE;
        self.board[3 + x][1 + y] = Cell::ALIVE;
        self.board[3 + x][2 + y] = Cell::ALIVE;
        self.board[2 + x][2 + y] = Cell::ALIVE;
        self.board[1 + x][1 + y] = Cell::ALIVE;
    }

    pub fn make_tile(&mut self, x: usize, y: usize) {
        self.board[x][y] = Cell::ALIVE;
    }

    pub fn make_line(&mut self, x: usize, y: usize, vert: bool) {
        if vert {
            self.board[x - 1][y] = Cell::ALIVE;
            self.board[x][y] = Cell::ALIVE;
            self.board[x + 1][y] = Cell::ALIVE;
        } else {
            self.board[x][y - 1] = Cell::ALIVE;
            self.board[x][y] = Cell::ALIVE;
            self.board[x][y + 1] = Cell::ALIVE;
        }
    }

    pub fn make_bottle(&mut self, x: usize, y: usize) {
        self.board[x + 4][y + 0] = Cell::ALIVE;
        self.board[x + 5][y + 0] = Cell::ALIVE;
        self.board[x + 5][y + 0] = Cell::ALIVE;
        self.board[x + 12][y + 0] = Cell::ALIVE;
        self.board[x + 13][y + 0] = Cell::ALIVE;

        self.board[x + 3][y + 1] = Cell::ALIVE;
        self.board[x + 6][y + 1] = Cell::ALIVE;
        self.board[x + 11][y + 1] = Cell::ALIVE;
        self.board[x + 14][y + 1] = Cell::ALIVE;

        self.board[x + 3][y + 2] = Cell::ALIVE;
        self.board[x + 5][y + 2] = Cell::ALIVE;
        self.board[x + 12][y + 2] = Cell::ALIVE;
        self.board[x + 14][y + 2] = Cell::ALIVE;

        self.board[x + 1][y + 3] = Cell::ALIVE;
        self.board[x + 2][y + 3] = Cell::ALIVE;
        self.board[x + 5][y + 3] = Cell::ALIVE;
        self.board[x + 6][y + 3] = Cell::ALIVE;
        self.board[x + 7][y + 3] = Cell::ALIVE;
        self.board[x + 10][y + 3] = Cell::ALIVE;
        self.board[x + 11][y + 3] = Cell::ALIVE;
        self.board[x + 12][y + 3] = Cell::ALIVE;
//        self.board[x + 13][y + 3] = Cell::ALIVE;
        self.board[x + 15][y + 3] = Cell::ALIVE;
        self.board[x + 16][y + 3] = Cell::ALIVE;

        self.board[x + 0][y + 4] = Cell::ALIVE;
        self.board[x + 7][y + 4] = Cell::ALIVE;
        self.board[x + 10][y + 4] = Cell::ALIVE;
        self.board[x + 17][y + 4] = Cell::ALIVE;

        self.board[x + 0][y + 5] = Cell::ALIVE;
        self.board[x + 2][y + 5] = Cell::ALIVE;
        self.board[x + 3][y + 5] = Cell::ALIVE;
        self.board[x + 14][y + 5] = Cell::ALIVE;
        self.board[x + 15][y + 5] = Cell::ALIVE;
        self.board[x + 17][y + 5] = Cell::ALIVE;

        self.board[x + 1][y + 6] = Cell::ALIVE;
        self.board[x + 3][y + 6] = Cell::ALIVE;
        self.board[x + 14][y + 6] = Cell::ALIVE;
        self.board[x + 16][y + 6] = Cell::ALIVE;

        self.board[x + 3][y + 7] = Cell::ALIVE;
        self.board[x + 4][y + 7] = Cell::ALIVE;
        self.board[x + 13][y + 7] = Cell::ALIVE;
        self.board[x + 14][y + 7] = Cell::ALIVE;

        ///////

        self.board[x + 3][y + 10] = Cell::ALIVE;
        self.board[x + 4][y + 10] = Cell::ALIVE;
        self.board[x + 13][y + 10] = Cell::ALIVE;
        self.board[x + 14][y + 10] = Cell::ALIVE;

        self.board[x + 1][y + 11] = Cell::ALIVE;
        self.board[x + 3][y + 11] = Cell::ALIVE;
        self.board[x + 14][y + 11] = Cell::ALIVE;
        self.board[x + 16][y + 11] = Cell::ALIVE;

        self.board[x + 0][y + 12] = Cell::ALIVE;
        self.board[x + 2][y + 12] = Cell::ALIVE;
        self.board[x + 3][y + 12] = Cell::ALIVE;
        self.board[x + 14][y + 12] = Cell::ALIVE;
        self.board[x + 15][y + 12] = Cell::ALIVE;
        self.board[x + 17][y + 12] = Cell::ALIVE;

        self.board[x + 0][y + 13] = Cell::ALIVE;
        self.board[x + 7][y + 13] = Cell::ALIVE;
        self.board[x + 10][y + 13] = Cell::ALIVE;
        self.board[x + 17][y + 13] = Cell::ALIVE;

        self.board[x + 1][y + 14] = Cell::ALIVE;
        self.board[x + 2][y + 14] = Cell::ALIVE;
        self.board[x + 5][y + 14] = Cell::ALIVE;
        self.board[x + 6][y + 14] = Cell::ALIVE;
        self.board[x + 7][y + 14] = Cell::ALIVE;
        self.board[x + 10][y + 14] = Cell::ALIVE;
        self.board[x + 11][y + 14] = Cell::ALIVE;
        self.board[x + 12][y + 14] = Cell::ALIVE;
 //       self.board[x + 13][y + 14] = Cell::ALIVE;
        self.board[x + 15][y + 14] = Cell::ALIVE;
        self.board[x + 16][y + 14] = Cell::ALIVE;

        self.board[x + 3][y + 15] = Cell::ALIVE;
        self.board[x + 5][y + 15] = Cell::ALIVE;
        self.board[x + 12][y + 15] = Cell::ALIVE;
        self.board[x + 14][y + 15] = Cell::ALIVE;

        self.board[x + 3][y + 16] = Cell::ALIVE;
        self.board[x + 6][y + 16] = Cell::ALIVE;
        self.board[x + 11][y + 16] = Cell::ALIVE;
        self.board[x + 14][y + 16] = Cell::ALIVE;

        self.board[x + 4][y + 17] = Cell::ALIVE;
        self.board[x + 5][y + 17] = Cell::ALIVE;
        self.board[x + 5][y + 17] = Cell::ALIVE;
        self.board[x + 12][y + 17] = Cell::ALIVE;
        self.board[x + 13][y + 17] = Cell::ALIVE;

    }

    pub fn rand(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                let num = rand::thread_rng().gen_bool(0.5);
                if num {self.board[i][j] = Cell::ALIVE}
                else {self.board[i][j] = Cell::DEAD}
            }
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
            let mut s = "".to_owned();
            for j in i {
                match j {
                    Cell::DEAD => s += "██",
                    Cell::ALIVE => s += "\x1b[32m██\x1b[0m",
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

impl From<Cell> for String {
    fn from(value: Cell) -> Self {
        match value {
            Cell::DEAD => "Dead",
            Cell::ALIVE => "Alive"
        }.to_owned()
    }
}
impl From<&Cell> for String {
    fn from(value: &Cell) -> Self {
        match value {
            &Cell::DEAD => "Dead",
            &Cell::ALIVE => "Alive"
        }.to_owned()
    }
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
            if k < 2 {Cell::DEAD}
            else if k == 2 || k == 3 {Cell::ALIVE}
            else {Cell::DEAD}
        } else {
            if k == 3 {Cell::ALIVE}
            else {Cell::DEAD}
        }
    }
}
