use plotters::prelude::*;

pub struct OutputLine {
  pub wheights: Vec<f32>,
  pub error_median: f32,
  pub accuracy: f32,
}

pub fn chart_output(output: Vec<OutputLine>) -> Result<(), Box<dyn std::error::Error>> {
  chart_error(&output)?;
  chart_accuracy(&output)?;

  Ok(())
}

fn chart_error(output: &Vec<OutputLine>) -> Result<(), Box<dyn std::error::Error>> {
  let canvas = SVGBackend::new("./output/error.svg", (1024, 768)).into_drawing_area();
  canvas.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&canvas)
    .set_label_area_size(LabelAreaPosition::Left, (8).percent())
    .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
    .margin((1).percent())
    .build_cartesian_2d(0..output.len(), 0.0..0.5f32)?;

  chart.configure_mesh().x_desc("EPOCH COUNT").y_desc("ERROR MEDIAN").draw()?;

  chart.draw_series(LineSeries::new(
    (0..output.len()).map(|i| (i, output[i].error_median)), &RED,
  ))?;

  canvas.present().expect("Unable to write chart to the file");

  Ok(())
}

fn chart_accuracy(output: &Vec<OutputLine>) -> Result<(), Box<dyn std::error::Error>> {
  let canvas = SVGBackend::new("./output/accuracy.svg", (1024, 768)).into_drawing_area();
  canvas.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&canvas)
    .set_label_area_size(LabelAreaPosition::Left, (8).percent())
    .set_label_area_size(LabelAreaPosition::Bottom, (4).percent())
    .margin((1).percent())
    .build_cartesian_2d(0..output.len(), 0.0..100.0f32)?;

  chart.configure_mesh().x_desc("EPOCH COUNT").y_desc("Accuracy %").draw()?;

  chart.draw_series(LineSeries::new(
    (0..output.len()).map(|i| (i, output[i].accuracy)), &RED,
  ))?;

  canvas.present().expect("Unable to write chart to the file");

  Ok(())
}