import numpy as np
import matplotlib.pyplot as plt

plt.style.use('seaborn-poster')

with open(".\\notes\\data3.txt", "r") as f:
    sr = 44100 * 4
    ts = 1 / sr * 4
    t = np.arange(0,4,ts)
    
    x = np.array([int(e.strip()) for e in f.readlines()])
    
    print(len(x))
    
    X = np.fft.fft(x)
    N = len(X)
    n = np.arange(N)
    T = N/sr
    freq = n/T 

    plt.figure(figsize = (12, 6))
    plt.subplot(121)

    plt.stem(freq, np.abs(X), 'b', \
            markerfmt=" ", basefmt="-b")
    plt.xlabel('Freq (Hz)')
    plt.ylabel('FFT Amplitude |X(freq)|')
    plt.xlim(0, sr / 2)

    plt.subplot(122)
    plt.plot(t, np.fft.ifft(X), 'r')
    plt.xlabel('Time (s)')
    plt.ylabel('Amplitude')
    plt.tight_layout()
    plt.show()