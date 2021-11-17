use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::{Seeder, SipHasher};

const WIDTH: usize = 128;
const HEIGHT: usize = 128;
const NAME: &str = "image.png";
const SEED: &str = "asdf";

pub fn main() {
    let rng: Pcg64 = Seeder::from("stripy zebra").make_rng();
    //let grid = grid(&checkerboard);
    let grid = grid(&white_noise);
    grid_to_image(grid, NAME);
}

fn checkerboard(x: usize, y: usize) -> u8 {
    if x % 2 == y % 2 {
        1
    } else {
        0
    }
}

fn r() -> u8{
    if &rng.gen_range(0..2) % 2 == 0 {0} else {1}
}

fn white_noise(x: usize, y: usize) -> u8 {
    //rand::thread_rng().gen_range(0..2)
}

fn grid(f: &dyn Fn(usize, usize) -> u8) -> [[u8; HEIGHT]; WIDTH] {
    let mut grid = [[0 as u8; HEIGHT]; WIDTH];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            grid[x][y] = f(x, y);
        }
    }

    grid
}

fn grid_to_image(array: [[u8; HEIGHT]; WIDTH], name: &str) {
    let mut image: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for x in 0..array.len() {
        for y in 0..array[x].len() {
            let xnum = x as u32;
            let ynum = y as u32;

            *image.get_pixel_mut(xnum, ynum) = if array[x][y] == 1 {
                image::Rgb([255, 255, 255])
            } else if array[x][y] == 0 {
                image::Rgb([0, 0, 0])
            } else {
                image::Rgb([127, 127, 127])
            }
        }
    }

    image.save(name).unwrap();
}
