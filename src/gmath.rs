use crate::Color;
use crate::Matrix;

//vector functions
//normalize vetor, should modify the parameter
pub fn normalize(vector: &mut Vec<f32>) {
  let mut m = 0.0;
  for i in 0..vector.len(){
    m += vector[i].powi(2);
  }
  m = m.sqrt();
  for i in 0..vector.len(){
    vector[i] /= m;
  }
}

// //Return the dot porduct of a . b
// fn dot_product(a, double *b ) {
//   return  0;
// }

impl Matrix{
  //Calculate the surface normal for the triangle whose first
  //point is located at index i in polygons
  pub fn calculate_normal(&self, i: usize) -> f32{
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
    let nx = ay * bz - az * by;
    let ny = az * bx - ax * bz;
    let nz = ax * by - ay * bx;
    let vx = 0.0;
    let vy = 0.0;
    let vz = 1.0;
    let dot_n_v = nx * vx + ny * vy + nz * vz;
    return dot_n_v;
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
  normal: f32,
) -> Color {
  let d: Color;
  return d;
}

// color calculate_specular(double light[2][3], double *sreflect, double *view, double *normal ) {

//   color s;
//   return s;
// }

//limit each component of c to a max of 255
// pub fn limit_color( color * c ) {
// }
