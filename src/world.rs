use rand::Rng;
use crate::util::Coord;
use crate::tree::*;
use crate::cell::*;
use crate::genome::*;

#[derive(Debug)]
pub struct WorldState {
    pub trees: Vec<Tree>,
    pub sun_intensity: (u32, u32),
    pub filled_space: Vec<Coord>,
    pub energy_matrix: Vec<Vec<u32>>, 
    pub falling_seeds: Vec<FallingSeed>,
    pub height: i32,
    pub width: i32,
    pub ground_energy: u32
}

impl WorldState {
    pub fn next_step(&mut self) {
        self.calulate_energy_matrix();
        self.propogate_trees();
    }

    pub fn calulate_energy_matrix(&mut self) {
        for x in 0..self.width {
            let mut energy = self.ground_energy;
            for y in 0..self.height {
                let xus: usize = x.try_into().unwrap();
                let yus: usize = y.try_into().unwrap();
                // println!("x:{}, y:{}, energy:{}", x, y, energy);
                self.energy_matrix[xus][yus] = energy;
                energy += 1;
            }
        }
    }

    fn propogate_trees(&mut self) {
        for mut tree in &mut self.trees[..] {
            let new_cells = Tree::propogate(&mut tree, &self.energy_matrix, &self.filled_space);
            Tree::branchify_shoots(&mut tree);
            for cell in new_cells {
                self.filled_space.push(cell.coord);
                tree.cells.push(cell);
            }
        }
    }

    pub fn add_trees(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            if 0 == rng.gen::<u8>()%3 {
                let mut genes = Vec::new();
                for _g in 0..32 {
                    genes.push(Gene {
                        up: rng.gen_range(0..32),
                        left: rng.gen_range(0..32),
                        right: rng.gen_range(0..32),
                        down: rng.gen_range(0..32)
                    })
                } 

                let new_tree = Tree {
                    cells: vec![Cell {
                        cell_type: CellType::Shoot,
                        gene: 0,
                        coord: Coord(x, 0)
                    }],
                    age: rng.gen_range(88..93),
                    energy: 0,
                    genome: Genome { genes },
                    colour: (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
                };

                self.filled_space.push(Coord(x, 0));
                self.trees.push(new_tree);
            }
        }
    }

    pub fn add_single_tree(&mut self) {
        let new_tree = Tree {
            cells: vec![Cell {
                cell_type: CellType::Shoot,
                gene: 0,
                coord: Coord(self.width/2, 0)
            }],
            age: 90,
            energy: 0,
            genome: Genome { genes: vec!(Gene { up: 7, left: 18, right: 27, down: 3 }, Gene { up: 8, left: 6, right: 9, down: 29 }, Gene { up: 27, left: 5, right: 31, down: 7 }, Gene { up: 28, left: 1, right: 28, down: 
                20 }, Gene { up: 16, left: 17, right: 9, down: 31 }, Gene { up: 11, left: 30, right: 8, down: 14 }, Gene { up: 1, left: 22, right: 6, down: 8 }, Gene { up: 4, left: 23, right: 12, down: 29 }, Gene { up: 28, left: 24, right: 8, down: 10 }, Gene { up: 11, left: 29, right: 6, down: 1 }, Gene { up: 28, left: 30, right: 19, down: 5 }, Gene { up: 7, left: 15, right: 31, down: 13 }, Gene { up: 10, left: 1, right: 29, down: 8 }, Gene { up: 
                22, left: 1, right: 18, down: 1 }, Gene { up: 2, left: 8, right: 0, down: 3 }, Gene { up: 2, left: 4, right: 2, down: 10 }, Gene { up: 25, left: 24, right: 3, down: 16 }, Gene { up: 26, left: 3, right: 6, down: 10 }, Gene { up: 29, left: 21, right: 23, down: 7 }, Gene { up: 5, left: 23, right: 10, down: 21 }, Gene { up: 19, left: 8, right: 21, down: 28 }, Gene { up: 25, left: 29, right: 16, down: 4 }, Gene { up: 6, left: 17, right: 
                31, down: 29 }, Gene { up: 9, left: 9, right: 10, down: 1 }, Gene { up: 16, left: 10, right: 25, down: 26 }, Gene { up: 15, left: 28, right: 24, down: 11 }, Gene { up: 0, left: 0, right: 16, down: 4 }, Gene { up: 29, left: 12, right: 10, down: 7 }, Gene { up: 2, left: 0, right: 26, down: 29 }, Gene { up: 29, left: 8, right: 4, down: 17 }, Gene { up: 16, left: 26, right: 2, down: 17 }, Gene { up: 8, left: 18, right: 2, down: 21 }) },
            colour: (0.15712118, 0.07359254, 0.7292814)
        };

        self.trees.push(new_tree);
    }
}
