mod data_utils;
mod perceptron_utils;
mod user_utils;
mod chart_utils;

use console::style;
use chart_utils::OutputLine;
use data_utils::read_data;
use perceptron_utils::*;

use crate::user_utils::print_out_config;

fn main() -> std::io::Result<()> {
  let (data_name, expected_size) = user_utils::select_dataset()?;
  let (epoch, learn_rate) = user_utils::select_vars()?;
  let activation = user_utils::select_activation()?;
  let mut wheights: Vec<f32> = Vec::new();
  let mut output: Vec<OutputLine> = Vec::new();

  let training_data = read_data(&format!("{data_name}training.data"), expected_size);
  let testing_data = read_data(&format!("{data_name}testing.data"), expected_size);

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

    output.push(OutputLine{
      wheights: wheights.clone(),
      error_median: squared_error/training_data.len() as f32
    });
  }

  // Final testing
  std::process::Command::new("clear").status().unwrap();
  println!("{}", style("Final wheights:").bold().yellow());
  for wheight in &wheights {
    print!("{} ", wheight)
  }
  println!("\n\n{}", style("Final stats using training data:").bold().yellow());
  test_wheights(&training_data, &wheights, &activation);
  println!("{}", style("\nFinal stats using testing data:").bold().yellow());
  test_wheights(&testing_data, &wheights, &activation);
  print_out_config(learn_rate, epoch, &data_name, activation(0.5) == 1.0);
  data_utils::write_stats(&output)?;
  match chart_utils::chart_output(output) {
    Ok(()) => print!(""),
    _ => println!("Failed to plot the chart")
  }

  Ok(())
}
