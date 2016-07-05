extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut imgbuf = image::ImageBuffer::new(400, 400);

    let red = [255, 0, 0];
    let blue = [0, 255, 0];
    let green = [0, 0, 255];
    let black = [0, 0, 0];

    let colors = [
        [red, blue, green, black],
        [blue, green, black, red],
        [green, black, red, blue],
        [black, red, blue, green],
    ];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgb = match (x, y) {
            (0...100, 0...100) => colors[0][0],
            (101...200, 0...100) => colors[0][1],
            (201...300, 0...100) => colors[0][2],
            (301...400, 0...100) => colors[0][3],

            (0...100, 101...200) => colors[1][0],
            (101...200, 101...200) => colors[1][1],
            (201...300, 101...200) => colors[1][2],
            (301...400, 101...200) => colors[1][3],

            (0...100, 201...300) => colors[2][0],
            (101...200, 201...300) => colors[2][1],
            (201...300, 201...300) => colors[2][2],
            (301...400, 201...300) => colors[2][3],

            (0...100, 301...400) => colors[3][0],
            (101...200, 301...400) => colors[3][1],
            (201...300, 301...400) => colors[3][2],
            (301...400, 301...400) => colors[3][3],

            (_, _) => unreachable!(),
        };

        *pixel = image::Rgb(rgb);
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
