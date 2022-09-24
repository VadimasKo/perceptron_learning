mod data_utils;
mod perceptron_utils;
use dialoguer::{ Select, theme::ColorfulTheme };
use console::{ Term, style };

fn main() -> std::io::Result<()> {
  std::process::Command::new("clear").status().unwrap();
  let options = vec!["train perceptron", "draw charts"];
      
  println!("{}", style("What do you want to do?").bold().yellow());
  let selection = Select::with_theme(&ColorfulTheme::default())
    .items(&options)
    .default(0)
    .interact_on_opt(&Term::stderr())?;

  match selection {
    Some(0) => {
      match perceptron_utils::run_training() {
        Err(err) => println!("{}", err),
        _ => println!("finished"),
      };
    },
    _ => print!("finished"),
  }
  Ok(())
}
