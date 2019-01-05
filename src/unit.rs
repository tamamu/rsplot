
use std::f64::consts::PI;
use std::cmp;

extern crate cairo;
use cairo::{ImageSurface, Format, Context};

pub type PlotUnit = f64;

static ROUNDING_ERROR: PlotUnit = 100.;

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

fn is_larger_than_one(a: f64) -> bool {
    (a - 1f64).is_sign_positive()
}

fn dec_pos(a: PlotUnit) -> usize {
    let mut x = a.abs();
    let mut pos: usize = 1;
    if is_larger_than_one(x) {
        pos = 0;
        while is_larger_than_one(x) {
            x /= 1e2;
            pos += 1;
        }
    }
    pos
}

pub fn round(l: PlotUnit, u: PlotUnit) -> (String, String) {
    let mut delta = u - l;
    let mut sig = 1;
    if (delta-1f64).is_sign_positive() {
        while (delta-1f64).is_sign_positive() {
            sig += 1;
            delta /= 2f64;
        }
    } else {
        while (delta-1f64).is_sign_negative() {
            sig += 1;
            delta *= 2f64;
        }
    }
    println!("{} {} ({})", l, u, sig);
    let mut ll = format!("{:0<1$}", (l * 1e2f64.powi(sig-1)).round() as i32, sig as usize);
    let mut uu = format!("{:0<1$}", (u * 1e2f64.powi(sig-1)).round() as i32, sig as usize);
    if ll.starts_with("-") {
        ll.truncate((sig+1) as usize);
        ll.insert(dec_pos(l)+1, '.');
    } else {
        ll.truncate(sig as usize);
        ll.insert(dec_pos(l), '.');
    }
    if uu.starts_with("-") {
        uu.truncate((sig+1) as usize);
        uu.insert(dec_pos(u)+1, '.');
    } else {
        uu.truncate(sig as usize);
        uu.insert(dec_pos(u), '.');
    }
    println!("{}, {}", ll, uu);

    (ll, uu)
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
pub struct Point2DProperty {
    pub data: Vec<Point2D>,
    pub lim_x: (PlotUnit, PlotUnit),
    pub lim_y: (PlotUnit, PlotUnit),
    pub color: Color,
}

impl Point2DProperty {
    pub fn new(data: Vec<Point2D>, unit_x: PlotUnit, unit_y: PlotUnit) -> Self {
        let mut lim_x = (0., 0.);
        let mut lim_y = (0., 0.);
        for p in &data {
            lim_x = (p.x.min(lim_x.0), p.x.max(lim_x.1));
            lim_y = (p.y.min(lim_y.0), p.y.max(lim_y.1));
        }
        Self {
            data: data,
            lim_x: lim_x,
            lim_y: lim_y,
            color: Color::RGB(0., 0., 0.),
        }
    }
}

pub type PointPlot = Plot2D<Point2DProperty>;


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

impl Drawable for Point2DProperty {
    fn draw(&self, ctx: &Context, width: f64, height: f64) {
        let ll_x = self.lim_x.0;
        let ul_x = self.lim_x.1;
        let ll_y = self.lim_y.0;
        let ul_y = self.lim_y.1;
        let ratio_x = width / (ul_x - ll_x);
        let ratio_y = height / (ul_y - ll_y);
        let (ll_xs, ul_xs) = round(ll_x, ul_x);
        let (ll_ys, ul_ys) = round(ll_y, ul_y);
        ctx.set_font_size(24.);
        ctx.move_to(-50., 0.);
        ctx.show_text(&ul_ys);
        ctx.move_to(-50., height);
        ctx.show_text(&ll_ys);
        ctx.move_to(0., height+50.);
        ctx.show_text(&ll_xs);
        ctx.move_to(width, height+50.);
        ctx.show_text(&ul_xs);
        round(ll_y, ul_y);
        set_context_color(ctx, self.color);
        /*
        for p in &self.data {
            println!("{:?}", p);
            ctx.arc((p.x-ll_x)*ratio_x, (ul_y-(p.y-ll_y))*ratio_y, 4f64, 0., 2.*PI);
            ctx.fill();
        }
        */
        let count = self.data.len();
        if (count >= 2) {
            let p = self.data[0];
            ctx.move_to((p.x-ll_x)*ratio_x, height-(p.y-ll_y)*ratio_y);
            for i in 1..self.data.len() {
                let p = self.data[i];
                ctx.line_to((p.x-ll_x)*ratio_x, height-(p.y-ll_y)*ratio_y);
            }
            ctx.stroke();
        }
    }
}
