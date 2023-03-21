use flo_draw::*;
use flo_draw::canvas::*;

mod world;
use world::WorldState;
mod tree;
mod cell;
mod util;
use crate::util::Coord;
mod genome;

const WORLD_HEIGHT: u32 = 10;
const WORLD_WIDTH: u32 = 10;
const TRANSPARENCY: u32 = 3;
const MAX_SUN_DEPTH: u32 = 6;
const GROUND_ENERGY: u32 = 1;
const PIXEL_SIZE: u32 = 10;



fn main() {
    with_2d_graphics(|| {
        let canvas: Canvas = create_canvas_window("Trees?");
        let world = init_world();

        canvas.draw(|gc| {
            gc.canvas_height((WORLD_HEIGHT * PIXEL_SIZE) as f32);
            gc.center_region(0.0, 0.0, (WORLD_WIDTH * PIXEL_SIZE) as f32, (WORLD_HEIGHT * PIXEL_SIZE) as f32);
        });

        canvas.draw(|gc| {
            gc.layer(LayerId(0));
            for x in 0..world.width {
                let mut energy = world.ground_energy;
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

        
        canvas.draw(|gc| {
            gc.layer(LayerId(1));
            for tree in world.trees {
                for cell in tree.cells {
                    let Coord(x, y) = cell.coord;
                    let red = tree.colour.0;
                    let blue = tree.colour.1;
                    let green = tree.colour.2;

                    println!("{:?}",cell.coord);
                    gc.new_path();
                    gc.rect((x * PIXEL_SIZE) as f32, (y * PIXEL_SIZE) as f32, (x * PIXEL_SIZE + PIXEL_SIZE) as f32, (y * PIXEL_SIZE + PIXEL_SIZE) as f32);
                    gc.fill_color(Color::Rgba(red, blue, green, 1.0));
                    gc.fill();
                }
            }
        });
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

    world.add_trees();

    world.calulate_energy_matrix();
    println!("{:?}", world);

    return world;
}
