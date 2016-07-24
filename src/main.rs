extern crate image;
extern crate rand;
extern crate csv;

use std::fs::File;
use std::path::Path;
use std::env;

#[derive(Clone)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB { red: red, green: green, blue: blue }
    }
}

fn read_colors(filepath: &str) -> Vec<RGB> {
    let mut csv_reader = csv::Reader::from_file(filepath).unwrap().has_headers(true);

    let mut colors: Vec<RGB> = vec![];

    for row in csv_reader.decode() {
        let (_name, red, green, blue): (String, u8, u8, u8) = row.unwrap();
        colors.push(RGB::new(red, green, blue));
    }

    colors
}

fn generate_tiles(colors: Vec<RGB>, tiles_per_row: usize) -> Vec<Vec<RGB>> {
    let mut tiles = vec![vec![RGB::new(0, 0, 0); tiles_per_row]; tiles_per_row];

    let mut rng = rand::thread_rng();

    for row in &mut tiles {
        for mut tile in row {
            *tile = rand::sample(&mut rng, colors.iter(), 1).pop().unwrap().clone();
        }
    }

    tiles
}

fn generate_image(image_size: u32, tile_size: u32, tiles: Vec<Vec<RGB>>) -> image::RgbImage {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_size, image_size);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let xx = x / tile_size;
        let yy = y / tile_size;

        let ref rgb = tiles[xx as usize][yy as usize];

        *pixel = image::Rgb([rgb.red, rgb.green, rgb.blue]);
    }

    imgbuf
}

fn save_image(image: image::RgbImage, filepath: &str) -> Result<(), image::ImageError> {
    let ref mut fout = File::create(&Path::new(filepath)).unwrap();

    image::ImageRgb8(image).save(fout, image::PNG)
}

fn create_image(size: u32,
                tiles_per_row: usize,
                colors_filepath: &str,
                output_filepath: &str)
                -> Result<(), image::ImageError> {
    let colors = read_colors(colors_filepath);

    let tiles = generate_tiles(colors, tiles_per_row);

    let tile_size = size / tiles_per_row as u32;

    let image = generate_image(size, tile_size, tiles);

    save_image(image, output_filepath)
}

fn main() {
    let mut args = env::args();

    let _program_name = args.next();

    let image_size: u32 = args
        .next()
        .expect("Image size required")
        .parse()
        .expect("Image size must be number");

    let tiles_per_row: usize = args
        .next()
        .expect("Tiles per row required")
        .parse()
        .expect("Tiles per row must be number");

    let _ = create_image(image_size, tiles_per_row, "./colors.csv", "./image.png");
}
