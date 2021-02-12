use std::cmp;
use ndarray::prelude::*;
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};
use apodize::{hamming_iter};

/// プロットに表示するxyの範囲
struct Extent { 
    xstart: f32, 
    xend: f32, 
    ystart: f32, 
    yend: f32
}

impl Extent {
    fn new(xstart: f32, xend: f32, ystart: f32, yend: f32) -> Self {
        Self {
            xstart: xstart, 
            xend: xend, 
            ystart: ystart, 
            yend: yend
        }
    }
}

/// startからendをサンプリング周波数rateで区切る
fn sampling_axis(start: f32, end: f32, rate: f32) -> Array1<f32> {
    Array1::range(start, end, 1.0 / rate)
}

/// プロットのラップ
fn plot<Iter1, Iter2>(xs: Iter1, ys: Iter2, file_name: &str, ex: Extent) -> Result<(), Box<dyn std::error::Error>> 
where 
    Iter1: Iterator<Item = f32>,
    Iter2: Iterator<Item = f32>
{
    let root = BitMapBackend::new(file_name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(ex.xstart..ex.xend, ex.ystart..ex.yend)?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(xs.zip(ys), &RED))?;

    Ok(())
}

/// ハミング窓
fn hamming(frame_len: usize) -> Array1<f32> {
    hamming_iter(frame_len).map(|x| x as f32).collect::<Array1<f32>>()
}

/// オーディオデータを保持するラッパー
struct Audio {
    data: Vec<f32>
}

impl Audio {
    /// オーディオデータのframe_lenの長さを、hop_lenずつずらしながら取得するイテレータを取得する。
    fn frame_iter(&self, frame_len: usize, hop_len: usize) -> Frames {
        Frames { 
            data: &self.data, 
            frame_len: frame_len, 
            hop_len: hop_len, 
            i: 0
        }
    }
}

struct Frames<'a> {
    data: &'a Vec<f32>,
    frame_len: usize,
    hop_len: usize,
    i: usize
}

/// データのframe_lenの長さを、hop_lenずつずらしながら取得するイテレータ
impl<'a> Iterator for Frames<'a> {
    type Item = Vec<f32>;

    fn next(&mut self) -> Option<Vec<f32>> {
        if self.i < self.data.len() {
            let end = cmp::min(self.i+self.frame_len, self.data.len());
            let audio_frame = &self.data[self.i..end];

            let mut frame = vec![0.0; self.frame_len];
            frame[..audio_frame.len()].copy_from_slice(audio_frame);

            self.i += self.hop_len;

            Option::Some(frame)
        } else {
            Option::None
        }
    }
}

fn frames(n: usize, frame_len: usize, hop_len: usize) {
    let a = Array1::range(0., n as f32, 1.);

    // STFTの結果格納用
    let mut m = Array2::<f32>::zeros((n / hop_len, frame_len));

    // ハミング窓
    let window = hamming(frame_len);

    // 行ごとにアサインしていく
    for (i, mut row) in m.axis_iter_mut(Axis(0)).enumerate() {
        let end = cmp::min(i*hop_len+frame_len, a.len());

        
        let mut fft_buf = Array1::<f32>::zeros(frame_len);
        {
            let slice_a = &a.slice(s![i*hop_len..end]);
            let mut buf_slice = fft_buf.slice_mut(s![..slice_a.len()]);
            buf_slice.assign(slice_a);
        }
        // 窓関数適用
        fft_buf *= &window;

        // TODO to_vecからのFFT
        let ffted = fft_buf;
        row.assign(&ffted);
    }
    println!("{:?}", m);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    frames(8, 3, 2);

    // // STFT用のイテレータを確認用
    // audio_frame();

    // // sinの合成波
    // let n_fft = 2048;
    // let times = sampling_axis(0.0, 1.0, n_fft as f32);
    // let w = 2.0 * std::f32::consts::PI;
    // let a = times.mapv(|x| 1.0 * (w * x * 10.0).sin());
    // let b = times.mapv(|x| 2.0 * (w * x * 20.0).sin());
    // let c = times.mapv(|x| 3.0 * (w * x * 30.0).sin());

    // let wave = a + b + c;
    // plot(times.iter().map(|x| *x), wave.iter().map(|x| *x), "output/sin.png", Extent::new(0.0, 1.0, -10.0, 10.0))?;

    // // FFTのプロット
    // plot_fft(&wave, n_fft, "output/fft_sin.png")?;

    // // 窓関数のプロット
    // let window = hamming(n_fft);
    // plot(times.iter().map(|x| *x), window.iter().map(|x| *x), "output/hamming.png", Extent::new(0.0, 1.0, 0.0, 1.0))?;

    // let windowed = window * wave;
    // plot(times.iter().map(|x| *x), windowed.iter().map(|x| *x), "output/windowed_sin.png", Extent::new(0.0, 1.0, -10.0, 10.0))?;

    // // 窓関数適用後のFFTのプロット
    // plot_fft(&windowed, n_fft, "output/fft_windowed_sin.png")?;

    Ok(())
}

fn plot_fft(signal: &Array1<f32>, n_fft: usize, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n_fft);
    let mut signal = signal.mapv(|x| Complex{ re: x, im: 0.0 }).to_vec();
    fft.process(&mut signal);

    let xs = (0..n_fft).map(|x| x as f32);
    let ys = signal.iter().map(|x| x.norm() / (signal.len() as f32).sqrt());
    plot(xs, ys, file_name, Extent::new(0.0, n_fft as f32 / 2.0, 0.0, 100.0))?;

    Ok(())
}

fn audio_frame() {
    let audio = Audio{ data: vec![1., 2., 3., 4., 5., 6.] };
    let frame_len = 3;
    let hop_len = 1;
    let audio_frames = audio.frame_iter(frame_len, hop_len);
    for frame in audio_frames {
        println!("{:?}", frame);
    }
}
