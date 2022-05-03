use color::Color;
use image::Image;
use matrix::CurveType;
use matrix::Matrix;
use parser::parse_file;
use std::env;
use reflect::ReflectionValue;
use rand::Rng;
mod gmath;
mod color;
mod draw;
mod image;
mod matrix;
mod parser;
mod reflect;

pub mod consts{
    use crate::color::Color;
    use crate::reflect::ReflectionValue; 

    pub const ambient: Color = Color::new_color(50, 50, 50);
    pub const ambient_reflect: ReflectionValue = ReflectionValue::new_values(0.1, 0.1, 0.1);
    pub const direct_reflect: ReflectionValue = ReflectionValue::new_values(0.5, 0.5, 0.5);
    pub const specular_reflect: ReflectionValue = ReflectionValue::new_values(0.5, 0.5, 0.5);
    pub const point_light_location: Vec<f32> = vec![0.5, 0.75, 1.0];
    pub const point_light_color: Color = Color::new_color(0, 255, 255);
    pub const view: Vec<f32> = vec![0.0,0.0,1.0];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut screen = Image::new(500, 500);
    let color = Color::new_color(0, 255, 0);
    let mut edges = Matrix::new(0, 0);
    let mut polygons = Matrix::new(0, 0);
    let mut cstack = vec![Matrix::new(0,0); 0];
    if args.len() > 1 && args[1] == "art" {
        parse_file(
            "marill",
            &mut cstack,
            &mut edges,
            &mut polygons,
            &mut screen,
            color,
        );
    } else {
        parse_file(
            "script",
            &mut cstack,
            &mut edges,
            &mut polygons,
            &mut screen,
            color,
        );
    }
}
