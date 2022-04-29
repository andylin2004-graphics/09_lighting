use crate::Color;

// //vector functions
// //normalize vetor, should modify the parameter
// fn normalize() {
// }

// //Return the dot porduct of a . b
// fn dot_product(a, double *b ) {
//   return  0;
// }


// //Calculate the surface normal for the triangle whose first
// //point is located at index i in polygons
// fn calculate_normal(Matrix polygons, i: i32) {
// }

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

pub fn get_lighting(normal: f32, view: f32, ambient_light: Color, point_light: &mut Vec<Vec<f32>>, ambient_reflect: f32, diffuse_reflect: f32, specular_reflect: f32) {
  // color i;
  // return i;
}

pub fn calculate_ambient(ambient_light: Color, ambient_reflect: f32) -> Color{
  let r = (ambient_light.r as f32 * ambient_reflect) as u8;
  let g = (ambient_light.g as f32 * ambient_reflect) as u8;
  let b = (ambient_light.b as f32 * ambient_reflect) as u8;
  let a = Color::new_color((ambient_light.r as f32 * ambient_reflect) as u8, (ambient_light.g as f32 * ambient_reflect) as u8, (ambient_light.b as f32 * ambient_reflect) as u8);
  return a;
}

pub fn calculate_diffuse(diffuse_light: &mut Vec<Vec<f32>>, diffuse_reflect: f32, normal: f32 ) -> Color{
  let d: Color;
  return d;
}

// color calculate_specular(double light[2][3], double *sreflect, double *view, double *normal ) {

//   color s;
//   return s;
// }

//limit each component of c to a max of 255
pub fn limit_color( color * c ) {
}