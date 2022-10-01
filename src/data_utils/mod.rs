use std::fs;

pub struct DataLine {
  pub x: Vec<f32>,
  pub t: f32
}

pub fn read_data(file_name: &str, expected_size: i32) -> Vec<DataLine> {
  let file_path = format!("./data/{file_name}");
  let mut data: Vec<DataLine> = Vec::new();

  let file_content = fs::read_to_string(&file_path)
    .expect(&format!{"File not found {}", &file_path});

  for file_line in file_content.split("\n").collect::<Vec<&str>>() {
    let mut inputs = file_line.split(",").filter_map(| s | s.parse::<f32>().ok()).collect::<Vec<_>>();
    
    if inputs.len() as i32 == expected_size {
      let class = inputs.pop().expect("Corrupted Line");
      let data_line = DataLine{ x: inputs, t: class};
      data.push(data_line)
    }
  }
  
  return data;
}
