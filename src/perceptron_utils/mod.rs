use crate::data_utils::DataLine;

pub fn step_activation(a: f32) -> f32 {
  return if a >= 0.0 { 1.0 } else { 0.0 } 
} 

pub fn sigmoid_activation(a: f32) -> f32 {
  let e = std::f32::consts::E;
  return 1.0 / (1.0 + e.powf(-a));
}

pub fn adeline(wheights: &mut Vec<f32>,line: &DataLine, output:f32, learn_rate: f32) {
  let gradient = 2.0 * (line.t - output);
  for i in 1..wheights.len() {
    // handle bias
    if i == 0 {
      wheights[0] += gradient * learn_rate;
    } else {
      wheights[i] += gradient * line.x[i-1] * learn_rate; 
    }

  }
}

pub fn calc_y(wheights: &Vec<f32>, inputs: &Vec<f32>) -> f32 {
  let mut sum: f32 = 1.0;
  for i in 0..wheights.len() {
    if i == 0 { sum *= wheights[i] }
    else { sum += wheights[i] * inputs[i-1] }
  }
  return sum;
}