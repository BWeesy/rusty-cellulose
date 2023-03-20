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
}

#[derive(Debug)]
pub enum CellType {
    Branch,
    Shoot,
    Seed
}
#[derive(Debug)]
pub struct Coord(i32, i32);

#[derive(Debug)]
pub struct Gene (i32, i32, i32, i32);

#[derive(Debug)]
pub struct Genome {
    genes: Vec<Gene>
}
#[derive(Debug)]
pub struct FallingSeed {
    genome: Genome,
    coord: Coord
}
#[derive(Debug)]
pub struct Cell {
    cell_type: CellType,
    gene: u32,
    coord: Coord
}
#[derive(Debug)]
pub struct Tree {
    cells: Vec<Cell>,
    age: i32,
    energy: i32,
    genome: Genome
}