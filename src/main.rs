use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;
const NAME: &str = "image1.png";
const SEED: &str = "asdf";

pub fn main() {
    //let grid = grid(&checkerboard);
    let grid = white_noise();
    grid_to_image(grid, NAME);
}

fn white_noise() -> [[u8; HEIGHT]; WIDTH] {
    let mut grid = [[0 as u8; HEIGHT]; WIDTH];
    let mut rng: Pcg64 = Seeder::from(SEED).make_rng();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            grid[x][y] = rng.gen_range(0..2);
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

//fn grid(f: &dyn Fn(usize, usize) -> u8) -> [[u8; HEIGHT]; WIDTH] {
    //let mut grid = [[0 as u8; HEIGHT]; WIDTH];

    //for y in 0..HEIGHT {
        //for x in 0..WIDTH {
            //grid[x][y] = f(x, y);
        //}
    //}

    //grid
//}

//fn checkerboard(x: usize, y: usize) -> u8 {
    //if x % 2 == y % 2 {
        //1
    //} else {
        //0
    //}
//}

