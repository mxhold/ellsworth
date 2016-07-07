extern crate image;
extern crate rand;
extern crate csv;

use std::fs::File;
use std::path::Path;

fn main() {
    let image_size = 400;
    let tiles_per_row: usize = 10;

    let tile_size = image_size / tiles_per_row as u32;

    let mut imgbuf = image::ImageBuffer::new(image_size, image_size);

    let mut csv_reader = csv::Reader::from_file("./colors.csv")
        .unwrap()
        .has_headers(true);

    let mut colors: Vec<[u8; 3]> = vec![];

    for row in csv_reader.decode() {
        let (_name, red, green, blue): (String, u8, u8, u8) = row.unwrap();
        colors.push([red, green, blue]);
    }

    let mut tiles = vec![vec![[0; 3]; tiles_per_row]; tiles_per_row];

    let mut rng = rand::thread_rng();

    for row in &mut tiles {
        for mut tile in row {
            *tile = *rand::sample(&mut rng, colors.iter(), 1).pop().unwrap();
        }
    }

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let xx = x / tile_size;
        let yy = y / tile_size;

        let rgb = tiles[xx as usize][yy as usize];

        *pixel = image::Rgb(rgb);
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
