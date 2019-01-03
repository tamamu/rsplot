
use std::f64::consts::PI;
use std::cmp;

extern crate cairo;
use cairo::{ImageSurface, Format, Context};

pub type PlotUnit = f64;

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: PlotUnit,
    pub y: PlotUnit,
}

#[derive(Debug, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
}

fn set_context_color(ctx: &Context, color: Color) {
    match color {
        Color::RGB(r, g, b) => {
            ctx.set_source_rgb(r, g, b);
        }
        Color::RGBA(r, g, b, a) => {
            ctx.set_source_rgba(r, g, b, a);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BorderProperty {
    pub color: Color,
    pub width: f64,
}

impl BorderProperty {
    fn new(color: Color, width: f64) -> Self {
        Self {
            color: color,
            width: width,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plot2D<T> {
    pub prop: T,
    pub title: String,
    pub label_x: String,
    pub label_y: String,
    pub border: BorderProperty,
    pub x: PlotUnit,
    pub y: PlotUnit,
    pub width: i32,
    pub height: i32,
}

impl<T> Plot2D<T> {
    pub fn new(prop: T) -> Self {
        Self {
            prop: prop,
            title: "".to_string(),
            label_x: "".to_string(),
            label_y: "".to_string(),
            border: BorderProperty::new(Color::RGB(0., 0., 0.), 2.),
            x: 0.,
            y: 0.,
            width: 800,
            height: 800,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scatter2DProperty {
    pub data: Vec<Point2D>,
    pub unit_x: PlotUnit,
    pub unit_y: PlotUnit,
    pub lim_x: (PlotUnit, PlotUnit),
    pub lim_y: (PlotUnit, PlotUnit),
    pub color: Color,
}

impl Scatter2DProperty {
    pub fn new(data: Vec<Point2D>, unit_x: PlotUnit, unit_y: PlotUnit) -> Self {
        let mut lim_x = (0., 0.);
        let mut lim_y = (0., 0.);
        for p in &data {
            lim_x = (p.x.min(lim_x.0), p.x.max(lim_x.1));
            lim_y = (p.y.min(lim_y.0), p.y.max(lim_y.1));
        }
        Self {
            data: data,
            unit_x: unit_x,
            unit_y: unit_y,
            lim_x: lim_x,
            lim_y: lim_y,
            color: Color::RGB(0., 0., 0.),
        }
    }
}

pub type Scatter2D = Plot2D<Scatter2DProperty>;


pub trait Drawable {
    fn draw(&self, ctx: &Context, width: f64, height: f64) {
        unimplemented!();
    }
}

impl<T: Drawable> Plot2D<T> {
    pub fn render(&self, ctx: &Context) {
        let w = self.width as f64;
        let h = self.height as f64;
        ctx.translate(self.x, self.y);
        let color = self.border.color;
        set_context_color(ctx, color);
        ctx.set_line_width(self.border.width);
        ctx.move_to(0., 0.);
        ctx.line_to(w, 0.);
        ctx.line_to(w, h);
        ctx.line_to(0., h);
        ctx.line_to(0., 0.);
        ctx.stroke();
        self.prop.draw(ctx, w, h);
    }
}

impl Drawable for Scatter2DProperty {
    fn draw(&self, ctx: &Context, width: f64, height: f64) {
        let ll_x = self.lim_x.0;
        let ul_x = self.lim_x.1;
        let ll_y = self.lim_y.0;
        let ul_y = self.lim_y.1;
        let ratio_x = width / (ul_x - ll_x);
        let ratio_y = height / (ul_y - ll_y);
        ctx.set_font_size(24.);
        ctx.move_to(-50., 0.);
        ctx.show_text(&ul_x.to_string());
        ctx.move_to(-50., height);
        ctx.show_text(&ll_x.to_string());
        ctx.move_to(0., height+50.);
        ctx.show_text(&ll_y.to_string());
        ctx.move_to(width, height+50.);
        ctx.show_text(&ul_y.to_string());
        set_context_color(ctx, self.color);
        for p in &self.data {
            println!("{:?}", p);
            ctx.arc((p.x-ll_x)*ratio_x, (ul_y-(p.y-ll_y))*ratio_y, 4f64, 0., 2.*PI);
            ctx.fill();
        }
    }
}
