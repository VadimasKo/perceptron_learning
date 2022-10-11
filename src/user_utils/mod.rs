use crate::perceptron_utils::{ step_activation, sigmoid_activation };
use dialoguer::{ Select, theme::ColorfulTheme };
use console::{ Term, style };
use text_io::*;


pub fn select_dataset() -> std::io::Result<(String, i32)> {
  std::process::Command::new("clear").status().unwrap();
  let options = vec!["breastCancer", "iris"];
      
  println!("{}", style("Select dataset:").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) => return Ok(("breastCancer/".to_owned(), 10)),
    _ => return Ok(("iris/".to_owned(), 5)),
  }
}

pub fn select_vars() -> std::io::Result<(i32, f32)> {
  std::process::Command::new("clear").status().unwrap();
  let options = vec![
    "epoch: 100, learn: 1.0",
    "epoch: 100, learn: 0.1",
    "epoch: 1000, learn: 0.01",
    "Custom Input"
  ];
      
  println!("{}", style("Select number of epochs and learning rate:").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(1) => Ok((100, 0.1)),
    Some(2) => Ok((1000, 0.01)),
    Some(3) => enter_custom_vars(),
    _ => Ok((100, 1.0))
  }
}

pub fn enter_custom_vars() -> std::io::Result<(i32, f32)> {
  std::process::Command::new("clear").status().unwrap();
  println!("{}", style("Enter epoch count:").bold().yellow());
  let epoch: i32 = read!();

  println!("{}", style("Enter learning rate:").bold().yellow());
  let learn_rate: f32 = read!();

  Ok((epoch, learn_rate))
}


pub fn select_activation() -> std::io::Result<fn(f32) -> f32> {
  std::process::Command::new("clear").status().unwrap();
  let options = vec!["Step activation", "Sigmoid activation"];
      
  println!("{}", style("Select Activation function:").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) =>  Ok(step_activation),
    _       =>  Ok(sigmoid_activation)
  }
}

pub fn print_out_config(learn_rate: f32, epoch: i32, data_name: &str, using_step: bool) {
  let activation = if using_step { "Step function" } else { "Sigmoid function" };

  println!("{}", style("\nConfiguration:").bold().yellow());
  println!("Data set:        {}", style(data_name).yellow());
  println!("Activation func: {}", style(activation).yellow());
  println!("Learning Rate:   {}", style(format!("{}", learn_rate)).yellow());
  println!("Nr of cycles:    {}", style(format!("{}", epoch)).yellow());
}
