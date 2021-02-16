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

fn stft(data: Array1<f32>, frame_len: usize, hop_len: usize) -> Array2::<f32> {
    // STFTの結果格納用
    let mut stft_frames = Array2::<f32>::zeros((data.len() / hop_len, frame_len));

    // ハミング窓
    let window = hamming(frame_len);
    let mut planner = FftPlanner::new();
    let fft_func = planner.plan_fft_forward(frame_len);

    // 行ごとにアサインしていく
    for (i, mut ffted_frame) in stft_frames.axis_iter_mut(Axis(0)).enumerate() {
        let windowed_frame = {
            let end = cmp::min(i*hop_len+frame_len, data.len());
            let data_slice = &data.slice(s![i*hop_len..end]);
            
            let mut frame = Array1::<f32>::zeros(frame_len);
            frame
            .slice_mut(s![..data_slice.len()])
            .assign(data_slice);

            // 窓関数適用
            frame *= &window;
            frame
        };

        // FFT
        let ffted = fft(&fft_func, windowed_frame);

        ffted_frame.assign(&ffted);
    }
    
    stft_frames
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = array![1.0, 2.0, 3.0];
    stft(a, 3, 2);

    Ok(())
}
