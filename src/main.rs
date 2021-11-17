use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::ops::RangeInclusive;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const NAME: &str = "image1.png";
const SEED: &str = "asdf";

//const HCorner: usize = HEIGHT - 1;
//const WCorner: usize = WIDTH - 1;
const _H: usize = HEIGHT - 2;
const _W: usize = WIDTH - 2;

const UPPER_LEFT: (usize,usize) = (0, 0);
const UPPER_RIGHT: (usize,usize) = (WIDTH - 1, 0);
const LOWER_LEFT: (usize,usize) = (0, HEIGHT - 1);
const LOWER_RIGHT: (usize,usize) = (0, HEIGHT - 1);

const TOP_EDGE: (RangeInclusive<usize>, usize) = (1..=_W, 0);
const BOTTOM_EDGE: (RangeInclusive<usize>, usize) = (1..=_W, HEIGHT);
const LEFT_EDGE: (usize, RangeInclusive<usize>) = (0, 1..=_H);
const RIGHT_EDGE: (usize, RangeInclusive<usize>) = (WIDTH, 1..=_H);

pub fn main() {
    //let grid = grid(&checkerboard);
    let mut grid = white_noise();

    //for _i in 0..3 {
        //grid = cellular_automata_pass(grid);
    //}

    grid_to_image(grid, NAME);
}

fn cellular_automata_pass(mut noise: [[u8; HEIGHT]; WIDTH]) -> [[u8; HEIGHT]; WIDTH] {
    //let mut grid = noise.clone();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            noise[x][y] = 0;

            let pair = (x, y);

            let neighbors :[u8; 8] = match pair{
                //fill
                (1..=_W, 1..=_H) =>
                    [
                        noise[x - 1][y - 1], //Upper left
                        noise[x][y + 1], //Upper mid
                        noise[x + 1][y + 1], //Upper right

                        noise[x - 1][y], //Mid left
                        noise[x + 1][y], //Mid right

                        noise[x - 1][y - 1], //Lower left
                        noise[x][y - 1], //Lower mid
                        noise[x + 1][y - 1], //Lower right
                    ],
                //corners
                UPPER_LEFT =>
                    [
                        2, //Upper left
                        2, //Upper mid
                        2, //Upper right

                        2, //Mid left
                        noise[x + 1][y], //Mid right

                        2, //Lower left
                        noise[x][y - 1], //Lower mid
                        noise[x + 1][y - 1], //Lower right
                    ],
                UPPER_RIGHT =>
                    [
                        2, //Upper left
                        2, //Upper mid
                        2, //Upper right

                        noise[x - 1][y], //Mid left
                        2, //Mid right

                        noise[x - 1][y - 1], //Lower left
                        noise[x][y - 1], //Lower mid
                        2, //Lower right
                    ],
                LOWER_LEFT =>
                    [
                        2, //Upper left
                        noise[x][y + 1], //Upper mid
                        noise[x + 1][y + 1], //Upper right

                        2, //Mid left
                        noise[x + 1][y], //Mid right

                        2, //Lower left
                        2, //Lower mid
                        2, //Lower right
                    ],
                LOWER_RIGHT =>
                    [
                        noise[x - 1][y - 1], //Upper left
                        noise[x][y + 1], //Upper mid
                        2, //Upper right

                        noise[x - 1][y], //Mid left
                        2, //Mid right

                        2, //Lower left
                        2, //Lower mid
                        2, //Lower right
                    ],
//edges
                TOP_EDGE =>
                    [
                        noise[x - 1][y - 1], //Upper left
                        noise[x][y + 1], //Upper mid
                        noise[x + 1][y + 1], //Upper right

                        noise[x - 1][y], //Mid left
                        noise[x + 1][y], //Mid right

                        noise[x - 1][y - 1], //Lower left
                        noise[x][y - 1], //Lower mid
                        noise[x + 1][y - 1], //Lower right
                    ],
                BOTTOM_EDGE =>
                    [
                        noise[x - 1][y - 1], //Upper left
                        noise[x][y + 1], //Upper mid
                        noise[x + 1][y + 1], //Upper right

                        noise[x - 1][y], //Mid left
                        noise[x + 1][y], //Mid right

                        noise[x - 1][y - 1], //Lower left
                        noise[x][y - 1], //Lower mid
                        noise[x + 1][y - 1], //Lower right
                    ],
                LEFT_EDGE =>
                    [
                        noise[x - 1][y - 1], //Upper left
                        noise[x][y + 1], //Upper mid
                        noise[x + 1][y + 1], //Upper right

                        noise[x - 1][y], //Mid left
                        noise[x + 1][y], //Mid right

                        noise[x - 1][y - 1], //Lower left
                        noise[x][y - 1], //Lower mid
                        noise[x + 1][y - 1], //Lower right
                    ],
                RIGHT_EDGE =>
                    [
                        noise[x - 1][y - 1], //Upper left
                        noise[x][y + 1], //Upper mid
                        noise[x + 1][y + 1], //Upper right

                        noise[x - 1][y], //Mid left
                        noise[x + 1][y], //Mid right

                        noise[x - 1][y - 1], //Lower left
                        noise[x][y - 1], //Lower mid
                        noise[x + 1][y - 1], //Lower right
                    ],
            };

            //grid[x][y] = ;
        }
    }

    noise
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
            //*image.get_pixel_mut(x as u32, y as u32) = image::Rgb([x.try_into().unwrap(), y.try_into().unwrap(), 0]);
            *image.get_pixel_mut(x as u32, y as u32) = if array[x][y] == 1 {
                image::Rgb([255, 255, 255])
            } else if array[x][y] == 0 {
                image::Rgb([0, 0, 0])
            } else {
                image::Rgb([255, 0, 0])
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
