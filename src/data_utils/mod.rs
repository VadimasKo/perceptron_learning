use std::fs;

pub fn read_cancer(file_name: &str) -> Vec<Vec<f64>> {
  let mut file_path = "./data/breastCancer/".to_string();
  file_path.push_str(file_name);

  println!("reading {}...", file_path);

  let file_content = fs::read_to_string(&file_path)
    .expect(&format!{"File not found {}", &file_path});
  
  let mut cancer_data: Vec<Vec<f64>> = Vec::new();
  
  for file_line in file_content.split("\n").collect::<Vec<&str>>() {
    let mut data_line: Vec<f64>= Vec::new();
    for input in file_line.split(",").collect::<Vec<&str>>() {
      let parsed_input: f64 = match input.parse() {
        Ok(v) => v,
        Err(_) => 999.0,
      };
      if parsed_input != 999.0 {
        data_line.push(parsed_input)
      }
    }

    if data_line.len() == 10 {
      cancer_data.push(data_line);
    } else {
      println!("\t ...!skiped faulty data Line");
    }
  }

  return cancer_data;
}

pub fn read_iris(file_name: &str) -> Vec<Vec<f64>> {
  let mut file_path = "./data/iris/".to_string();
  file_path.push_str(file_name);

  println!("reading {}...", file_path);

  let file_content = fs::read_to_string(&file_path)
    .expect(&format!{"File not found {}", &file_path});
  
  let mut cancer_data: Vec<Vec<f64>> = Vec::new();
  
  for file_line in file_content.split("\n").collect::<Vec<&str>>() {
    let mut data_line: Vec<f64>= Vec::new();
    for input in file_line.split(",").collect::<Vec<&str>>() {
      let parsed_input: f64 = match input.parse() {
        Ok(v) => v,
        Err(_) => 999.0,
      };
      if parsed_input != 999.0 {
        data_line.push(parsed_input)
      }
    }

    if data_line.len() == 5 {
      cancer_data.push(data_line);
    } else {
      println!("Skiped faulty data Line");
    }
  }

  return cancer_data;
}

