use crate::util::Coord;

use crate::cell::*;
use crate::genome::Genome;

#[derive(Debug)]
pub struct Tree {
    pub cells: Vec<Cell>,
    pub age: i32,
    pub energy: i32,
    pub genome: Genome,
    pub colour: (f32, f32, f32)
}

impl Tree {
    fn init(&mut self, coord: Coord) {
        self.cells.push(Cell {
            cell_type: CellType::Shoot,
            gene: 0,
            coord: coord
        })
    }
}

#[derive(Debug)]
pub struct FallingSeed {
    genome: Genome,
    coord: Coord
}