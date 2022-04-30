use crate::Color;
use crate::Matrix;
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

pub fn calculate_ambient(ambient_light: Color, ambient_reflect: f32) -> Color {
  let r = (ambient_light.r as f32 * ambient_reflect) as u8;
  let g = (ambient_light.g as f32 * ambient_reflect) as u8;
  let b = (ambient_light.b as f32 * ambient_reflect) as u8;
  let a = Color::new_color(
    (ambient_light.r as f32 * ambient_reflect) as u8,
    (ambient_light.g as f32 * ambient_reflect) as u8,
    (ambient_light.b as f32 * ambient_reflect) as u8,
  );
  return a;
}

pub fn calculate_diffuse(
  diffuse_light_location: &mut Vec<f32>,
  diffuse_light_color: Color,
  diffuse_reflect: f32,
  normal: &mut Vec<f32>,
) -> Color {
  // let d: Color::new_color(r: u8, g: u8, b: u8);
  normalize(diffuse_light_location);
  normalize(normal);
  let n_l_dot_product_times_reflect = dot_product(normal, diffuse_light_location) * diffuse_reflect;
  let color = Color::new_color(
    ((n_l_dot_product_times_reflect * diffuse_light_color.r as f32) as i32 % 256) as u8,
    ((n_l_dot_product_times_reflect * diffuse_light_color.g as f32) as i32 % 256) as u8,
    ((n_l_dot_product_times_reflect * diffuse_light_color.b as f32) as i32 % 256) as u8,
  );
  return color;
}

// color calculate_specular(double light[2][3], double *sreflect, double *view, double *normal ) {

//   color s;
//   return s;
// }

//limit each component of c to a max of 255
// pub fn limit_color( color * c ) {
// }
