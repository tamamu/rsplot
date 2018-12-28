
extern crate rsplot;
use rsplot::unit::{Point2D};

extern crate cairo;
use cairo::{ImageSurface, Format, Context};

use std::fs::File;

fn main() {
    let p = Point2D {x: 3., y: 4.};
    println!("Hello, world!{:?}", p);
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)
        .expect("Couldn't create a surface!");
    let context = Context::new(&surface);
    context.set_source_rgb(1.0, 0.0, 0.0);
    context.paint();

    let mut file = File::create("output.png")
        .expect("Couldn't create a file!");

    surface.write_to_png(&mut file)
        .expect("Couldn't write to png!");
}
