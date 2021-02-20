#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}
fn main() {
    println!("Hello, world!");
}
