use crate::perceptron_utils::{ step_activation, sigmoid_activation };
use dialoguer::{ Select, theme::ColorfulTheme };
use console::{ Term, style };


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
    Some(1) => Ok((100, 0.01)),
    Some(2) => Ok((100, 0.001)),
    _ => Ok((100, 0.1))
  }
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
