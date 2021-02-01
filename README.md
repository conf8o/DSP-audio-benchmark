# DSP_audio_benchmark
音声ファイル取得からMFCC(メル周波数ケプストラム係数)への変換までの各言語のベンチマーク

レスポンス速度が求められるGUIアプリを作ろうとしているので、速度と作りやすさの両方を見て言語を決めたい。

SIMD命令でFFTをするライブラリがある言語が良い。GPUは考えない。

## 候補
* C++
* C#
* F#
* ~~Java~~
* Python
* Rust
* Swift

## それぞれの良い点と懸念点

観点
* 速度
* 作りやすさ
  * エコシステム
  * 学習コスト
  * 保守性
    
言語   | 良い点                                      | 懸念点
---    | ---                                         | ---
C++    | 最速、豊富なライブラリ                      | 一番むずい、だるい
C#     | .NET、資料多い、Xamarinの可能性             | 特になし(慣れてない、だるい、最速ではない)
F#     | 関数型、.NET、おもしろい、Xamarinの可能性   | 資料が少ない、慣れてない
Python | 慣れてる、一番簡単、資料多い                | 速度は微妙かも(NumPyでも)、 保守性が微妙
Rust   | 最速、安全、おもしろい                      | GUIが微妙らしい、むずかしい、慣れてない
Swift  | 慣れてる、楽しい、Apple限定で作るならこれ   | Apple限定(vDSPフレームワーク)

## 考察

.NET系は使い慣れていないので少し抵抗感あり、使えるようになってしまえばエコシステムと保守性が良いのでアリ。
C++, Rustはハードルが高いけど本気でやるならベストな選択。
SwiftはApple限定でなければ本当に最高。
Pythonは一番簡単だけど遅いかも。

## 決め方

取り組むべき順に並び替えた。

言語   | 決め方
---    | ---
Python | 正直これが一番楽。ベンチマークを取ってみて他のとそこまで差が出ないならこれにする
Rust   | 最速値取得用。やってみて案外行けそうと思ったらこれにする
===    | === 壁 ===
F#     | Xamarinにワンチャン懸けたい気持ちがある。速度が出ていてかつSwiftより速いならこれにする。
Swift  | Apple限定なら最有力候補なのでやってみる。速度が思ったより出ないなら諦める。
C#     | C#やるならF#やりたい。
===    | === 壁 ===
C++    | 一番抵抗感が大きい。

## ベンチマークの取り方

バッファにとっておいた音声データからMFCCへの変換までの速度を計測する。計測するプログラムの作りやすさも感じておく。

### 条件

SwiftがいるのでmacOSで行う。

```
MacBook Pro (13-inch, 2018)
OS: macOS Big Sur
CPU: Intel Core i5-8259U @ 2.3 GHz (4 cores)
DRAM: 16GB 2,133MHz LPDDR3
GPU: Intel Iris Plus Graphics 655
```

言語ごとにベンチマーク測定プログラムを作成する。

ベンチマーク測定は複数回行い、平均と標準偏差を求める。

### MFCCを求める

MFCCを求めるための条件
* サンプリング周波数=16kHz
* 次元数=12
* FFTの要素数=1024(16kHzにおいては64ms)
* STFTのホップサイズ=256
* メルフィルタバンクの次元数=20
* DCTの定義=DCT-Ⅱ

1. ストリーミングのように、1分間のwavファイルを128ミリ秒のフレームに分けてバッファにとっておき、そのあと計測をスタートしてそれぞれMFCCを求める。
    * マイク入力を想定するので、while文で実装する。

なお、作りやすさも見たいのでパッケージ化を試みる。パッケージ名はテキトーに`audio.feature`としておく。

#### 注意点

なるべくオーバーヘッドが無いようにプログラムを組み立てる。その大変さも選定の要素に含める。(Python, Swiftあたりはその辺少し慣れているけど、Rustとか大変そう。)

長時間音声のベンチマークは取る必要ない。
