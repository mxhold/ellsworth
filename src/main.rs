extern crate image;
extern crate rand;
extern crate csv;

use std::fs::File;
use std::path::Path;
use std::env;

fn read_colors(filepath: &str) -> Vec<[u8; 3]> {
    let mut csv_reader = csv::Reader::from_file(filepath).unwrap().has_headers(true);

    let mut colors: Vec<[u8; 3]> = vec![];

    for row in csv_reader.decode() {
        let (_name, red, green, blue): (String, u8, u8, u8) = row.unwrap();
        colors.push([red, green, blue]);
    }

    colors
}

fn generate_tiles(colors: Vec<[u8; 3]>, tiles_per_row: usize) -> Vec<Vec<[u8; 3]>> {
    let mut tiles = vec![vec![[0; 3]; tiles_per_row]; tiles_per_row];

    let mut rng = rand::thread_rng();

    for row in &mut tiles {
        for mut tile in row {
            *tile = *rand::sample(&mut rng, colors.iter(), 1).pop().unwrap();
        }
    }

    tiles
}

fn generate_image(image_size: u32, tile_size: u32, tiles: Vec<Vec<[u8; 3]>>) -> image::RgbImage {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_size, image_size);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let xx = x / tile_size;
        let yy = y / tile_size;

        let rgb = tiles[xx as usize][yy as usize];

        *pixel = image::Rgb(rgb);
    }

    imgbuf
}

fn write_image_file(image: image::RgbImage, filepath: &str) -> Result<(), image::ImageError> {
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

    write_image_file(image, output_filepath)
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
