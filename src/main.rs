mod data_utils;
mod perceptron_utils;
mod user_utils;

use data_utils::read_data;
use perceptron_utils::*;

fn main() -> std::io::Result<()> {
  let (data_name, expected_size) = user_utils::select_dataset()?;
  let (epoch, learn_rate) = user_utils::select_vars()?;
  let activation = user_utils::select_activation()?;
  let mut wheights: Vec<f32> = Vec::new();

  let training_data = read_data(&format!("{data_name}training.data"), expected_size);
  let _testing_data = read_data(&format!("{data_name}testing.data"), expected_size);

  // initialize wheights
  for _i in 0..expected_size { wheights.push(1.0) }

  // neuoron training
  for _i in 0..epoch {
    let mut squared_error = 0.0;

    for data_line in &training_data {
      let y = calc_y(&wheights, &data_line.x);
      let output = activation(y);
      squared_error += (output - data_line.t).powi(2);

      if output != data_line.t {
        adeline(&mut wheights, data_line, output, learn_rate);
      }
    }

    println!("error rate: {}", squared_error/training_data.len() as f32)
  }

  Ok(())
}
