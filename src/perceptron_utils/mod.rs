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

pub fn is_class_correct(class: f32, t: f32) -> bool {
  return class >= 0.5 && t == 1.0 || class < 0.5 && t == 0.0;
}

pub fn test_wheights(data: &Vec<DataLine>, wheights: &Vec<f32>, activation: &dyn Fn(f32) -> f32) -> (f32, f32){
  let mut correct_count = 0.0;
  let mut squared_error = 0.0;

  for data_line in data {
    let y = calc_y(wheights, &data_line.x);
    let output = activation(y);
    squared_error += (output - data_line.t).powi(2);

    if is_class_correct(output, data_line.t) { correct_count += 1.0 }
  }

  let accuracy = correct_count/data.len() as f32 * 100.0;
  let error_median = squared_error/data.len() as f32;

  return (accuracy, error_median)
}