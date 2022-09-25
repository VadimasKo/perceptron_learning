use dialoguer::{ Select, theme::ColorfulTheme };
use console::{ Term, style };
use super::activation::*;

pub fn select_dataset() -> std::io::Result<String> {
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

pub fn select_vars(mut _epoch: i32, mut _learn_rate: f64) -> std::io::Result<()> {
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
      _epoch = 100;
    },
    Some(1) => {
      _epoch = 100;
      _learn_rate = 0.01;
    },
    _ => {
      _epoch = 100;
      _learn_rate = 0.001;
    }
  }
  Ok(())
}

pub fn select_activation(mut _f:&dyn Fn(f64) -> f64) -> std::io::Result<()>  {
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
