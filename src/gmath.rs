use crate::Color;
use crate::Matrix;
use crate::ReflectionValue;
use std::cmp;

//vector functions
//normalize vector, should modify the parameter
pub fn normalize(vector: &mut Vec<f32>) {
  let mut m = 0.0;
  for i in 0..vector.len() {
    m += vector[i].powi(2);
  }
  m = m.sqrt();
  for i in 0..vector.len() {
    vector[i] /= m;
  }
}

// Return the dot product of a . b
pub fn dot_product(lhs: &Vec<f32>, rhs: &Vec<f32>) -> f32 {
  let mut result = 0.0;
  for i in 0..cmp::min(lhs.len(), rhs.len()) {
    result += lhs[i] * rhs[i];
  }
  return result;
}

// modify the param vector when the vector is multiplied by a scalar
pub fn vector_times_scalar(vector: &mut Vec<f32>, scalar: f32) -> Vec<f32>{
  let mut result = vector.clone();
  for i in 0..result.len(){
    result[i] *= scalar;
  }
  return result;
}

pub fn vector_subtraction(lhs: &mut Vec<f32>, rhs: &mut Vec<f32>) -> Vec<f32>{
  let mut result = lhs.clone();
  for i in 0..result.len(){
    result[i] -= rhs[i];
  }
  return result;
}

impl Matrix {
  //Calculate the surface normal for the triangle whose first
  //point is located at index i in polygons
  pub fn calculate_normal(&self, i: usize) -> f32 {
    let x0 = self.matrix_array[0][i];
    let y0 = self.matrix_array[1][i];
    let z0 = self.matrix_array[2][i];
    let x1 = self.matrix_array[0][i + 1];
    let y1 = self.matrix_array[1][i + 1];
    let z1 = self.matrix_array[2][i + 1];
    let x2 = self.matrix_array[0][i + 2];
    let y2 = self.matrix_array[1][i + 2];
    let z2 = self.matrix_array[2][i + 2];
    let ax = x1 - x0;
    let ay = y1 - y0;
    let az = z1 - z0;
    let bx = x2 - x0;
    let by = y2 - y0;
    let bz = z2 - z0;
    let n = vec![ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx];
    let v = vec![0.0, 0.0, 1.0];
    return dot_product(&n, &v);
  }
}

impl Color{
  fn color_with_lighting(constant: f32, light_color: Color, reflect: ReflectionValue) -> Color{
    Color{
      r: (constant * light_color.r as f32 * reflect.r) as u8,
      g: (constant * light_color.g as f32 * reflect.g) as u8,
      b: (constant * light_color.b as f32 * reflect.b) as u8
    }
  }
}

/*============================================
IMPORANT NOTE

Ambient light is represeneted by a color value

Point light sources are 2D arrays of doubles.
     - The fist index (LOCATION) represents the vector to the light.
     - The second index (COLOR) represents the color.

Reflection constants (ka, kd, ks) are represened as arrays of
doubles (red, green, blue)
============================================*/

//lighting functions

pub fn get_lighting(
  normal: f32,
  view: f32,
  ambient_light: Color,
  point_light: &mut Vec<Vec<f32>>,
  ambient_reflect: f32,
  diffuse_reflect: f32,
  specular_reflect: f32,
) {
  // color i;
  // return i;
}

pub fn calculate_ambient(ambient_light: Color, ambient_reflect: ReflectionValue) -> Color {
  return Color::color_with_lighting(1.0, ambient_light, ambient_reflect);
}

pub fn calculate_diffuse(
  diffuse_light_vector: &mut Vec<f32>,
  diffuse_light_color: Color,
  diffuse_reflect: ReflectionValue,
  normal: &mut Vec<f32>,
) -> Color {
  normalize(diffuse_light_vector);
  normalize(normal);
  let n_l_dot_product_times = dot_product(normal, diffuse_light_vector);
  return Color::color_with_lighting(n_l_dot_product_times, diffuse_light_color, diffuse_reflect);
}

pub fn calculate_specular(specular_light_vector: &mut Vec<f32>, specular_light_color: Color, specular_reflect: ReflectionValue, view: &Vec<f32>, normal: &mut Vec<f32> ) -> Color{
  normalize(normal);
  normalize(specular_light_vector);
  let calculation_before_color_and_light = &mut vector_subtraction(&mut vector_times_scalar(normal, 2.0 * dot_product(normal, specular_light_vector)), specular_light_vector);
  let calculation_before_color = dot_product(calculation_before_color_and_light, view);
  return Color::color_with_lighting(calculation_before_color, specular_light_color, specular_reflect);
}
