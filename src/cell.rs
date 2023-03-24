use crate::util::Coord;

#[derive(Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub gene: u32,
    pub coord: Coord
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    Branch,
    Shoot,
    Seed
}