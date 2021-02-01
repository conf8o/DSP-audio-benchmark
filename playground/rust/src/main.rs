use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1024);

    let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; 1024];
    fft.process(&mut buffer);

}
