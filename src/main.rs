extern crate image;
extern crate rand;

use std::fs::File;
use std::path::Path;

fn main() {
    let image_size = 400;
    let tiles_per_row: usize = 25;

    let tile_size = image_size / tiles_per_row as u32;

    let mut imgbuf = image::ImageBuffer::new(image_size, image_size);

    let red = [255, 0, 0];
    let blue = [0, 255, 0];
    let green = [0, 0, 255];
    let black = [0, 0, 0];
    let cyan = [0, 255, 255];
    let magenta = [255, 0, 255];
    let yellow = [255, 255, 0];
    let white = [255, 255, 255];

    let colors = [red, blue, green, black, cyan, magenta, yellow];

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
