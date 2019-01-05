
extern crate rsplot;
use rsplot::unit::*;

extern crate cairo;
use cairo::{ImageSurface, Format, Context, Matrix};

use std::fs::File;

fn main() {
    let x: Vec<f64> = (0..629).into_iter().map(|i| i as f64).collect();
    let mut data: Vec<Point2D> = x.iter().map(|i| Point2D {x: *i*0.01, y: (i*0.01).sin()}).collect();
    data[0].y = 4.;
    round(1.0, 1.0001);
    /*
    let data = vec![
        Point2D {x: 0., y: 40.},
        Point2D {x: 50., y: 100.},
    ];
    */
    let prop = Point2DProperty::new(data, 1.0, 1.0);
    let mut plot = Plot2D::new(prop);
    plot.width = 400;
    plot.height = 400;
    plot.x = 100.;
    plot.y = 100.;
    let surface = ImageSurface::create(Format::ARgb32, plot.width+200, plot.height+200)
        .expect("Couldn't create a surface!");
    let context = Context::new(&surface);
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();
    context.set_source_rgb(0.0, 0.0, 0.0);
    plot.render(&context);

    let mut file = File::create("output.png")
        .expect("Couldn't create a file!");

    surface.write_to_png(&mut file)
        .expect("Couldn't write to png!");
}
