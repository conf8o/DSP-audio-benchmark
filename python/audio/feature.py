import numpy as np
import librosa


RATE = 16000
N_MFCC = 12
N_FFT = 1024
HOP_LENGHT = 256
N_MELS = 20


def mfcc(data: np.ndarray):
    feature = librosa.feature.mfcc(data, 
                                   RATE, 
                                   n_mfcc=N_MFCC, 
                                   n_fft=N_FFT,
                                   hop_length=HOP_LENGHT, 
                                   n_mels=N_MELS)

    return feature
