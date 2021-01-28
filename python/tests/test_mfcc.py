import time
import pytest
import audio.feature

# サンプル(https://blog.amedama.jp/entry/2019/05/08/235438)
# def something(duration=0.1):
#     time.sleep(duration)
#     return True


# def test_something_benchmark(benchmark):
#     ret = benchmark.pedantic(something,
#                              kwargs={'duration': 0.0001},  # テスト対象に渡す引数 (キーワード付き)
#                              rounds=100,  # テスト対象の呼び出し回数
#                              iterations=10)  # 試行回数
#     assert ret


# グローバルに音声データを取っておく


def test_mfcc_benchmark(benchmark):
    # グローバルな音声データに対してmfccを求める
    pass

if __name__ == '__main__':
    pytest.main(['-v', __file__])