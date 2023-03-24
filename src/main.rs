use cell::CellType;
use flo_draw::*;
use flo_draw::canvas::*;

mod world;
use world::WorldState;
mod tree;
mod cell;
mod util;
use crate::util::Coord;
mod genome;

use std::thread;
use std::time::Duration;

const WORLD_HEIGHT: i32 = 100;
const WORLD_WIDTH: i32 = 100; 
const TRANSPARENCY: u32 = 3;
const MAX_SUN_DEPTH: u32 = 6;
const GROUND_ENERGY: u32 = 1;
const PIXEL_SIZE: i32 = 10;



fn main() {
    with_2d_graphics(|| {
        let canvas: Canvas = create_canvas_window("Trees?");
        let mut world = init_world();

        canvas.draw(|gc| {
            gc.canvas_height((WORLD_HEIGHT * PIXEL_SIZE) as f32);
            gc.center_region(0.0, 0.0, (WORLD_WIDTH * PIXEL_SIZE) as f32, (WORLD_HEIGHT * PIXEL_SIZE) as f32);
        });

        loop {
            canvas.draw(|gc| {
                gc.layer(LayerId(0));
                for x in 0..world.width { 
                    for y in 0..world.height {
                        let xus: usize = x.try_into().unwrap();
                        let yus: usize = y.try_into().unwrap();
                        gc.new_path();
                        gc.rect((x * PIXEL_SIZE) as f32, (y * PIXEL_SIZE) as f32, (x * PIXEL_SIZE + PIXEL_SIZE) as f32, (y * PIXEL_SIZE + PIXEL_SIZE) as f32);
                        gc.fill_color(Color::Rgba(1.0, 1.0, 0.0, world.energy_matrix[xus][yus] as f32/WORLD_HEIGHT as f32));
                        gc.fill();
                    }
                }
            });
            let current_trees = &world.trees;
            canvas.draw(|gc| {
                gc.layer(LayerId(1));
                for tree in current_trees {
                    for cell in &tree.cells {
                        let Coord(x, y) = cell.coord;
                        let red = if cell.cell_type == CellType::Shoot { 1.0 } else { tree.colour.0 };
                        let blue = if cell.cell_type == CellType::Shoot { 1.0 } else { tree.colour.1 };
                        let green = if cell.cell_type == CellType::Shoot { 1.0 } else { tree.colour.2 };
    
                        gc.new_path();
                        gc.rect((x * PIXEL_SIZE) as f32, (y * PIXEL_SIZE) as f32, (x * PIXEL_SIZE + PIXEL_SIZE) as f32, (y * PIXEL_SIZE + PIXEL_SIZE) as f32);
                        gc.fill_color(Color::Rgba(red, blue, green, 1.0));
                        gc.fill();
                    }
                }
            });
            world.next_step();
            thread::sleep(Duration::from_secs(1));
        }

    });
}

fn init_world() -> WorldState {
    let mut world = WorldState {
        height: WORLD_HEIGHT,
        width: WORLD_WIDTH,
        energy_matrix: vec![vec![0; WORLD_HEIGHT.try_into().unwrap()]; WORLD_WIDTH.try_into().unwrap()],
        trees: Vec::new(),
        filled_space: Vec::new(),
        falling_seeds: Vec::new(),
        sun_intensity: (TRANSPARENCY, MAX_SUN_DEPTH),
        ground_energy: GROUND_ENERGY
    };

    // world.add_trees();
    world.add_single_tree();

    world.calulate_energy_matrix();
    println!("Height {:?}", world.height);
    println!("width {:?}", world.width);
    println!("sun_intensity {:?}", world.sun_intensity);
    println!("ground_energy {:?}", world.ground_energy);
    println!("trees {:?}", world.trees);
    println!("falling_seeds {:?}", world.falling_seeds);

    return world;
}
