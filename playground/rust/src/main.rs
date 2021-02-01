use ndarray::Array1;

fn main() {
    println!("{}", sin_wave(2., 16000, 100));
}

fn sin_wave(k: f64, rate: u32, ms: u32) -> Array1<f64> {
    let end = (ms as f64) / 1000.;
    let n = (rate * ms / 1000) as usize;
    let w = 2. * std::f64::consts::PI * k;

    (Array1::linspace(0., end, n) * w).mapv(f64::sin)
}
