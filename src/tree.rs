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

    pub fn branchify_shoots(&mut self) {
        for i in 0..self.cells.len() {
            if self.cells[i].cell_type == CellType::Shoot {
                self.cells[i].cell_type = CellType::Branch;
            }
        }
    }

    pub fn propogate(&mut self, energy_matrix: &Vec<Vec<u32>>, filledspace: &Vec<Coord>) -> Vec<Cell> {
        let mut new_cells = Vec::new();
        for cell in &self.cells {
            if cell.cell_type != CellType::Shoot { continue; }
            let gene = &self.genome.genes[cell.gene as usize];
            let up_neighbour = Coord(cell.coord.0, cell.coord.1 + 1);
            if gene.up < 16 && Self::is_allowed_new_space(&up_neighbour, filledspace) {
                new_cells.push(Cell {
                    cell_type: CellType::Shoot,
                    gene: gene.up,
                    coord: up_neighbour
                });
            }
            let left_neighbour = Coord(cell.coord.0 - 1, cell.coord.1);
            if gene.up < 16 && Self::is_allowed_new_space(&left_neighbour, filledspace) {
                new_cells.push(Cell {
                    cell_type: CellType::Shoot,
                    gene: gene.left,
                    coord: left_neighbour
                });
            }
            let right_neighbour = Coord(cell.coord.0 + 1, cell.coord.1);
            if gene.up < 16 && Self::is_allowed_new_space(&right_neighbour, filledspace) {
                new_cells.push(Cell {
                    cell_type: CellType::Shoot,
                    gene: gene.right,
                    coord: right_neighbour
                });
            }
            let down_neighbour = Coord(cell.coord.0, cell.coord.1 - 1);
            if gene.up < 16 && Self::is_allowed_new_space(&down_neighbour, filledspace) {
                new_cells.push(Cell {
                    cell_type: CellType::Shoot,
                    gene: gene.down,
                    coord: down_neighbour
                });
            }
        }
        new_cells
    }

    fn is_allowed_new_space(proposed: &Coord, filledspace: &Vec<Coord>) -> bool {
        Self::is_space_empty(proposed, filledspace) && Self::does_space_exist(proposed)
    }

    fn is_space_empty(proposed: &Coord, filledspace: &Vec<Coord>) -> bool {
        !filledspace.contains(proposed)
    }

    fn does_space_exist(proposed: &Coord) -> bool {
        if proposed.0 < 0 || proposed.1 < 0 {
            return false
        }

        true
    }
}

#[derive(Debug)]
pub struct FallingSeed {
    genome: Genome,
    coord: Coord
}