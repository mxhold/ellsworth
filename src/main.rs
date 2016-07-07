extern crate image;
extern crate rand;
extern crate csv;

use std::fs::File;
use std::path::Path;

fn read_colors(filepath: &str) -> Vec<[u8; 3]> {
    let mut csv_reader = csv::Reader::from_file(filepath)
        .unwrap()
        .has_headers(true);

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

fn main() {
    let image_size = 400;
    let tiles_per_row: usize = 10;
    let tile_size = image_size / tiles_per_row as u32;

    let colors = read_colors("./colors.csv");

    let tiles = generate_tiles(colors, tiles_per_row);

    let image = generate_image(image_size, tile_size, tiles);

    let _ = write_image_file(image, "image.png");
}
