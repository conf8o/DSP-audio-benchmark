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

fn audio_frame() {
    let audio = Audio{ data: vec![1., 2., 3., 4., 5., 6.] };
    let frame_len = 3;
    let hop_len = 1;
    let audio_frames = audio.frame_iter(frame_len, hop_len);
    for frame in audio_frames {
        println!("{:?}", frame);
    }
}
