use dialoguer::{ Select, theme::ColorfulTheme };
use console::{ Term, style };
use std::io;
use crate::data_utils;

fn step_activation(a: f64) -> f64 {
  return if a >= 0.0 { 1.0 } else { 0.0 } 
} 

fn sigmoid_activation(a: f64) -> f64 {
  let e = std::f64::consts::E;
  let result = 1.0 / (1.0 + e.powf(-a));
  return if result >= 0.5 { 1.0 } else { 0.0 }
}

pub fn run_training() -> std::io::Result<()> {
  let mut _data_name = String::new();
  let mut _output = String::new();
  let activation_fun = &step_activation;
  let epoch = 100.0;
  let learn_rate = 0.1;

  _data_name = select_dataset()?;
  select_activation(activation_fun)?;
  select_vars(epoch, learn_rate)?;
  _output = format!("{}_e{}_l{}.data", _data_name, epoch, learn_rate);

  print!("{} \n", _output);  

  let _training_set = match _data_name.as_str() {
    "cancer" => data_utils::read_cancer("training.data"),
    _ => data_utils::read_iris("training.data"),
  };

  // let testing_set = match selected {
  //   "cancer" => data_utils::read_cancer("training.data"),
  //   _ => data_utils::read_iris("training.data"),
  // };
  Ok(())
}

fn select_dataset() -> std::io::Result<String> {
  std::process::Command::new("clear").status().unwrap();
  let options = vec!["breastCancer.data", "iris.data"];
      
  println!("{}", style("Select dataset:").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) => return Ok("cancer".to_owned()),
    _ => return Ok("iris".to_owned()),
  }
}

fn select_activation(mut _f:&dyn Fn(f64) -> f64) -> std::io::Result<()>  {
  std::process::Command::new("clear").status().unwrap();
  let options = vec!["Step activation", "Sigmoid activation"];
      
  println!("{}", style("Select Activation function:").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) => _f = &step_activation,
    _ => _f = &sigmoid_activation,
  }
  Ok(())
}

fn select_vars(mut _epoch: f64, mut _learn_rate: f64) -> std::io::Result<()> {
  std::process::Command::new("clear").status().unwrap();
  let options = vec![
    "epoch: 100, learn: 0.1",
    "epoch: 100, learn: 0.01",
    "epoch: 100, learn: 0.001",
  ];
      
  println!("{}", style("Select number of epochs and learning rate:").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) => {
      _learn_rate = 0.1;
      _epoch = 100.0;
    },
    Some(1) => {
      _epoch = 100.0;
      _learn_rate = 0.01;
    },
    _ => {
      _epoch = 100.0;
      _learn_rate = 0.001;
    }
  }
  Ok(())
}
