mod activation;
mod user_inputs;

use crate::data_utils;
use activation::*;
use user_inputs::*;

pub fn run_training() -> std::io::Result<()> {
  let mut _data_name = String::new();
  let mut _output = String::new();
  let activation_fun = &step_activation;
  let epoch = 100;
  let learn_rate = 0.1;

  _data_name = select_dataset()?;
  select_activation(activation_fun)?;
  select_vars(epoch, learn_rate)?;
  _output = format!("{}_e{}_l{}.data", _data_name, epoch, learn_rate);

  let training_set = match _data_name.as_str() {
    "cancer" => data_utils::read_cancer("training.data"),
    _ => data_utils::read_iris("training.data"),
  };

  let mut wheights: Vec<f64> = Vec::new();
  
  while wheights.len() < training_set[0].len() {
    wheights.push(0.0);
  }

  for _i in 0..epoch {
    // println!("{}", i);
  }


  
  Ok(())
}
