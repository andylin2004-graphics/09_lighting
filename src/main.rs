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

    pub const AMBIENT_COLOR: Color = Color::new_color(50, 50, 50);
    pub const AMBIENT_REFLECT: ReflectionValue = ReflectionValue::new_values(0.1, 0.1, 0.1);
    pub const DIRECT_REFLECT: ReflectionValue = ReflectionValue::new_values(0.5, 0.5, 0.5);
    pub const SPECULAR_REFLECT: ReflectionValue = ReflectionValue::new_values(0.5, 0.5, 0.5);
    pub const POINT_LIGHT_LOCATION: [f32; 3] = [0.5, 0.75, 1.0];
    pub const POINT_LIGHT_COLOR: Color = Color::new_color(0, 255, 255);
    pub const VIEW: [f32; 3] = [0.0,0.0,1.0];
    pub const STEP_2D: i32 = 100;
    pub const STEP_3D: i32 = 100;
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
            "botsamirite",
            &mut cstack,
            &mut edges,
            &mut polygons,
            &mut screen,
            &color,
            &mut consts::VIEW.to_vec(),
            &mut consts::AMBIENT_COLOR,
            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
            &mut consts::POINT_LIGHT_COLOR,
            &mut consts::AMBIENT_REFLECT,
            &mut consts::SPECULAR_REFLECT,
            &mut consts::DIRECT_REFLECT
        );
    } else {
        parse_file(
            "script",
            &mut cstack,
            &mut edges,
            &mut polygons,
            &mut screen,
            &color,
            &mut consts::VIEW.to_vec(),
            &mut consts::AMBIENT_COLOR,
            &mut consts::POINT_LIGHT_LOCATION.to_vec(),
            &mut consts::POINT_LIGHT_COLOR,
            &mut consts::AMBIENT_REFLECT,
            &mut consts::SPECULAR_REFLECT,
            &mut consts::DIRECT_REFLECT
        );
    }
}
