use ndarray::Array1;
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};

struct Extent { 
    xstart: f32, 
    xend: f32, 
    ystart: f32, 
    yend:f32,
}

impl Extent {
    fn new(xstart: f32, xend: f32, ystart: f32, yend:f32) -> Self {
        Self {xstart: xstart, xend: xend, ystart: ystart, yend: yend}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sinの合成波
    let n_fft = 2048;
    let times = sampling_axis(0.0, 1.0, n_fft as f32);
    let w = 2. * std::f32::consts::PI;
    let a = times.mapv(|x| 1.0 * (w * x * 10.0).sin());
    let b = times.mapv(|x| 2.0 * (w * x * 20.0).sin());
    let c = times.mapv(|x| 3.0 * (w * x * 30.0).sin());

    let wave = a + b + c;
    plot(times.iter().map(|x| *x), wave.iter().map(|x| *x), "output/0.png", Extent::new(0.0, 1.0, -10.0, 10.0))?;

    // FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n_fft);
    let mut wave = wave.mapv(|x| Complex{ re: x, im: 0.0 }).to_vec();
    fft.process(&mut wave);

    let xs = (0..n_fft).map(|x| x as f32);
    let ys = wave.iter().map(|x| x.norm() / (wave.len() as f32).sqrt());
    plot(xs, ys, "output/1.png", Extent::new(0.0, n_fft as f32 / 2.0, 0.0, 100.0))?;

    Ok(())
}

fn sampling_axis(start: f32, end: f32, rate: f32) -> Array1<f32> {
    Array1::range(start, end, 1. / rate)
}

fn plot<Iter1, Iter2>(xs: Iter1, ys: Iter2, file_name: &str, ex: Extent) -> Result<(), Box<dyn std::error::Error>> 
where 
    Iter1: Iterator<Item = f32>,
    Iter2: Iterator<Item = f32>,
{
    let root = BitMapBackend::new(file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(ex.xstart..ex.xend, ex.ystart..ex.yend)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(xs.zip(ys), &RED))?;

    Ok(())
}
