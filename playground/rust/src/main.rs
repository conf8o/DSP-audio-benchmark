use ndarray::Array1;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("output/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0f32..1f32, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    let times = sampling_axis(0., 1., 16000.);
    let w = 2. * std::f32::consts::PI * 10.;
    chart.draw_series(LineSeries::new(times.iter().map(|x| (*x, (w * x).sin())), &RED))?;
    Ok(())
}

fn sampling_axis(start: f32, end: f32, rate: f32) -> Array1<f32> {
    Array1::range(start, end, 1. / rate)
}
