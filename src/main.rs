mod data_utils;
use data_utils::*;

fn main() {
  let iris_data = read_iris("training.data");
  let cancer_data = read_cancer("training.data");

}
