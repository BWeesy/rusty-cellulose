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
    pub height: u32,
    pub width: u32,
    pub ground_energy: u32
}

impl WorldState {
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

    pub fn add_trees(&mut self) {
        let mut rng = rand::thread_rng();
        for x in 0..self.width {
            if 0 == rng.gen::<u8>()%3 {
                let mut genes = Vec::new();
                for _g in 0..32 {
                    genes.push(Gene(rng.gen_range(0..32), rng.gen_range(0..32), rng.gen_range(0..32), rng.gen_range(0..32)))
                } 

                let first_shoot = Cell {
                    cell_type: CellType::Shoot,
                    gene: 0,
                    coord: Coord(x, 0)
                };

                let new_tree = Tree {
                    cells: vec![first_shoot],
                    age: rng.gen_range(88..93),
                    energy: 0,
                    genome: Genome { genes },
                    colour: (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
                };

                self.trees.push(new_tree);
            }
        }
    }
}
