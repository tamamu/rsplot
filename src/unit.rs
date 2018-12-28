
pub type PlotUnit = f32;

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
    RGB(f32, f32, f32),
    RGBA(f32, f32, f32, f32),
}

#[derive(Debug, Clone, Copy)]
pub struct BorderProperty {
    pub color: Color,
    pub width: u32,
}

impl BorderProperty {
    fn new(color: Color, width: u32) -> Self {
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
}

impl<T> Plot2D<T> {
    pub fn new(prop: T) -> Self {
        Self {
            prop: prop,
            title: "".to_string(),
            label_x: "".to_string(),
            label_y: "".to_string(),
            border: BorderProperty::new(Color::RGB(0., 0., 0.), 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scatter2DProperty {
    pub data: Vec<Point2D>,
    pub unit_x: PlotUnit,
    pub unit_y: PlotUnit,
}

impl Scatter2DProperty {
    pub fn new(data: Vec<Point2D>, unit_x: PlotUnit, unit_y: PlotUnit) -> Self {
        Self {
            data: data,
            unit_x: unit_x,
            unit_y: unit_y,
        }
    }
}

pub type Scatter2D = Plot2D<Scatter2DProperty>;
