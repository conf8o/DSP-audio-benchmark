use std::cmp;
use ndarray::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex, Fft};
use apodize::{hamming_iter};

/// ハミング窓
fn hamming(frame_len: usize) -> Array1<f32> {
    hamming_iter(frame_len).map(|x| x as f32).collect::<Array1<f32>>()
}

/// FFT
fn fft(fft_func: &std::sync::Arc<dyn Fft<f32>>, signal: Array1<f32>) -> Array1<f32> {
    let mut ffted = signal.mapv(|x| Complex{ re: x, im: 0.0 }).to_vec();
    fft_func.process(&mut ffted);
        
    ffted
    .iter()
    .map(|x| x.norm() / (ffted.len() as f32).sqrt())
    .collect::<Array1<f32>>()
}

struct Stft {
    fft_func: std::sync::Arc<dyn Fft<f32>>,
    window: Array1<f32>,
    hop_len: usize
}

impl Stft {
    fn new(frame_len: usize, hop_len: usize) -> Stft {
        let mut planner = FftPlanner::new();
        let fft_func = planner.plan_fft_forward(frame_len);

        Stft{ fft_func: fft_func, window: hamming(frame_len), hop_len: hop_len }
    }

    fn process(&self, data: Array1<f32>) -> Array2::<f32> {
        let frame_len = self.window.len();

        // STFTの結果格納用
        let mut stft_frames = Array2::<f32>::zeros((data.len() / self.hop_len, frame_len));

        // 行ごとに代入していく
        for (i, mut ffting_frame) in stft_frames.axis_iter_mut(Axis(0)).enumerate() {
            let windowed_frame = {
                let mut frame = Array1::<f32>::zeros(frame_len);

                let end = cmp::min(i * self.hop_len + frame_len, data.len());
                let data_slice = &data.slice(s![(i*self.hop_len)..end]);
                
                // 窓関数適用
                frame
                .slice_mut(s![..data_slice.len()])
                .assign(&(data_slice * &self.window));

                frame
            };

            // FFT
            let ffted = fft(&self.fft_func, windowed_frame);

            ffting_frame.assign(&ffted);
        }
    
        stft_frames
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stft = Stft::new(3, 2);

    let a = array![1.0, 2.0, 3.0];

    println!("{}", stft.process(a));
    Ok(())
}
