fn main() {
    println!("Hello, world!");

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
