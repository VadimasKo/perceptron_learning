pub fn step_activation(a: f64) -> f64 {
  return if a >= 0.0 { 1.0 } else { 0.0 } 
} 

pub fn sigmoid_activation(a: f64) -> f64 {
  let e = std::f64::consts::E;
  let result = 1.0 / (1.0 + e.powf(-a));
  return if result >= 0.5 { 1.0 } else { 0.0 }
}
