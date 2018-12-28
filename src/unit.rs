
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
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, f32),
}

#[derive(Debug, Clone, Copy)]
pub struct BorderProperty {
    pub color: Color,
    pub width: u32,
}

#[derive(Debug, Clone)]
pub struct Plot2D<T> {
    pub data: T,
    pub title: String,
    pub label_x: String,
    pub label_y: String,
    pub border: BorderProperty,
}

#[derive(Debug, Clone)]
pub struct Scatter2DProperty {
    pub data: Vec<Point2D>,
    pub unit_x: PlotUnit,
    pub unit_y: PlotUnit,
}

pub type Scatter2D = Plot2D<Scatter2DProperty>;
