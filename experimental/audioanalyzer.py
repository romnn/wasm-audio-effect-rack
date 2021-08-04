import math
import numpy as np
import melbank
from SimpleWebSocketServer import SimpleWebSocketServer, WebSocket
from scipy.ndimage.filters import gaussian_filter1d


class ExpFilter:
    """Simple exponential smoothing filter"""

    def __init__(self, val=0.0, alpha_decay=0.5, alpha_rise=0.5):
        """Small rise / decay factors = more smoothing"""
        assert 0.0 < alpha_decay < 1.0, "Invalid decay smoothing factor"
        assert 0.0 < alpha_rise < 1.0, "Invalid rise smoothing factor"
        self.alpha_decay = alpha_decay
        self.alpha_rise = alpha_rise
        self.value = val

    def update(self, value):
        if isinstance(self.value, (list, np.ndarray, tuple)):
            alpha = value - self.value
            alpha[alpha > 0.0] = self.alpha_rise
            alpha[alpha <= 0.0] = self.alpha_decay
        else:
            alpha = self.alpha_rise if value > self.value else self.alpha_decay
        self.value = alpha * value + (1.0 - alpha) * self.value
        return self.value


MIN_VOLUME_THRESHOLD = 1e-7
N_FRAMES_ROLLING_WINDOW = 2
N_FRAMES_ROLLING_WINDOW = 1

N_FFT_BINS = 24
MIN_FREQUENCY = 200
MAX_FREQUENCY = 12000

mel_gain = ExpFilter(np.tile(1e-1, N_FFT_BINS), alpha_decay=0.01, alpha_rise=0.99)
mel_smoothing = ExpFilter(np.tile(1e-1, N_FFT_BINS), alpha_decay=0.5, alpha_rise=0.99)
gain = ExpFilter(np.tile(0.01, N_FFT_BINS), alpha_decay=0.001, alpha_rise=0.99)


class AudioProcessor:
    def __init__(self, sample_rate, fps, nchannels):
        self.sample_rate = sample_rate
        self.fps = fps
        self.nchannels = nchannels

        self.samples_per_frame = int(self.sample_rate / self.fps)
        print(self.samples_per_frame * N_FRAMES_ROLLING_WINDOW)

        self.fft_window = np.hamming(self.samples_per_frame * N_FRAMES_ROLLING_WINDOW)

        self.roll_win = (
            np.zeros((N_FRAMES_ROLLING_WINDOW, self.samples_per_frame)) / 1e16
        )

        mel_samples = int(self.sample_rate * N_FRAMES_ROLLING_WINDOW / (2.0 * self.fps))
        print("frames_in_rolling_window", N_FRAMES_ROLLING_WINDOW)
        print("sample_rate", self.sample_rate)
        print("fps", self.fps)
        print("mel_samples", mel_samples)

        # print("sample rate", self.sample_rate)
        # print("fft bins", N_FFT_BINS)
        # mel samples 735
        # sample rate 44100
        # fft bins 24

        # mel_x is the fft_frequencies
        # mel_y are the weights?
        self.mel_y, (_, self.mel_x) = melbank.compute_melmat(
            num_mel_bands=N_FFT_BINS,
            freq_min=MIN_FREQUENCY,
            freq_max=MAX_FREQUENCY,
            num_fft_bands=mel_samples,
            sample_rate=self.sample_rate,
        )

        self.visualization_effect = self.visualize_scroll

    def visualize_scroll(self, y):
        y = y ** 2.0
        gain.update(y)
        y /= gain.value
        y *= 255.0
        r = int(np.max(y[: len(y) // 3]))
        g = int(np.max(y[len(y) // 3 : 2 * len(y) // 3]))
        b = int(np.max(y[2 * len(y) // 3 :]))
        return (r, g, b)
        # Scrolling effect window
        p[:, 1:] = p[:, :-1]
        p *= 0.98
        p = gaussian_filter1d(p, sigma=0.2)
        # Create new color originating at the center
        p[0, 0] = r
        p[1, 0] = g
        p[2, 0] = b
        # Update the LED strip
        return np.concatenate((p[:, ::-1], p), axis=1)

    def process(self, data):
        if self.nchannels == 1:
            stereo = np.array([data, data])
        else:
            stereo = np.array([data[::2], data[1::2]])

        # combine stereo signal to mono
        mono = np.amax(stereo, axis=0)
        if mono.shape[0] != self.samples_per_frame:
            return None
            # raise ValueError("mono shape[0] does not match samples per frame")
        # normalize
        mono = mono / 2 ** 15

        # add to rolling window
        self.roll_win[:-1] = self.roll_win[1:]
        self.roll_win[-1, :] = np.copy(mono)
        window = np.concatenate(self.roll_win, axis=0).astype(np.float32)
        assert len(window) == len(self.fft_window)

        volume = np.max(np.abs(mono))
        if volume < MIN_VOLUME_THRESHOLD:
            # ignore for now
            pass
        else:
            window_size = len(window)

            # pad with zeros until the next power of two
            next_power_of_two = int(np.ceil(np.log2(window_size)))
            padding = 2 ** next_power_of_two - window_size

            window *= self.fft_window
            window = np.pad(window, (0, padding), mode="constant")
            ys = np.abs(np.fft.rfft(window)[: window_size // 2])
            print("ys", ys.shape)
            print("ys (2d)", np.atleast_2d(ys).T.shape)  # (735, 1)
            print("mel.T", self.mel_y.T.shape)  # (735, 1)

            # construct a Mel filterbank from the FFT data
            mel = np.atleast_2d(ys).T * self.mel_y.T

            # scale data to values more suitable for visualization
            mel = np.sum(mel, axis=0)
            mel = mel ** 2.0

            # gain normalization
            mel_gain.update(np.max(gaussian_filter1d(mel, sigma=1.0)))
            mel /= mel_gain.value
            mel = mel_smoothing.update(mel)

            return self.visualization_effect(mel)


def gen_samples(length, func):
    return np.array([func(i) for i in range(length)]).reshape((length, 1))


def test_gen_function(x):
    return 0.7 * np.sin(x + 100) + 0.3 * np.sin(x)


if __name__ == "__main__":
    # print("we will do the tests here that will be great!")
    from numpy.testing import assert_almost_equal

    sample_rate = 44100
    fps = 60
    nchannels = 1

    # assume already mono
    sample_count = 735
    samples = gen_samples(sample_count, test_gen_function)
    print(samples.shape)
    assert_almost_equal(
        samples[:4, 0], np.array([-0.35445595, 0.56885935, 0.96916798, 0.47842804])
    )
    # make abs
    samples = np.abs(samples)

    # make mono
    samples = np.amax(samples, axis=1)
    print(samples.shape, len(samples))

    volume = np.max(samples)
    print("volume", volume)
    # assert_almost_equal(volume, 0.969167981998589)

    window_size = len(samples)
    # pad with zeros until the next power of two
    next_power_of_two = int(np.ceil(np.log2(window_size)))
    print("po2", next_power_of_two)
    padding = 2 ** next_power_of_two - window_size
    print("padding", padding)

    # normally we have a fixed amount of samples per frame and we choose it to be able to make 60 analyses per seconds which is the speed at which the vis should run
    # however, we do not (yet) have that in rust and we therefore use the test window we have right here
    samples_per_frame = int(sample_rate / fps)
    samples_per_frame = len(samples)
    frames_in_rolling_window = 1
    fft_window = np.hamming(samples_per_frame * frames_in_rolling_window)

    print("window size", len(samples))
    print("fft_window", len(fft_window))
    print("before hamming", samples[:5].reshape((-1, 1)))
    samples *= fft_window
    print("after hamming", samples[:5].reshape((-1, 1)))

    window = np.pad(samples, (0, padding), mode="constant")
    print("window", len(window), window[:5].reshape((-1, 1)))

    ys = np.abs(np.fft.rfft(window)[: window_size // 2])
    print("fft ys", ys.shape)
    print("fft ys", ys[:5].reshape((-1, 1)))

    # construct a Mel filterbank from the FFT data
    mel_samples = int(sample_rate * frames_in_rolling_window / (2.0 * fps))
    # mel_samples = int(samples_per_frame * frames_in_rolling_window / (2.0 * fps))
    # mel_samples = 1
    print("frames_in_rolling_window", frames_in_rolling_window)
    print("sample_rate", sample_rate)
    print("fps", fps)
    print("mel_samples", mel_samples)

    fft_bins = 24
    min_freq = 200
    max_freq = 12000
    # mel samples 735
    # sample rate 44100
    # fft bins 24

    mel_y, (_, mel_x) = melbank.compute_melmat(
        num_mel_bands=fft_bins,
        freq_min=min_freq,
        freq_max=max_freq,
        num_fft_bands=mel_samples,
        sample_rate=sample_rate,
    )

    a = np.atleast_2d(ys).T
    print("np.atleast_2d(ys).T", a.shape, a[:5, :])
    b = mel_y.T
    print("mel_y.T", b.shape, b[:5, :])
    print("mel_y.T ( min %f max %f )" % (np.min(b), np.max(b)))
    mel = a * b
    print("mel", mel.shape, mel[:5, :])
    print("mel ( min %f max %f )" % (np.min(mel), np.max(mel)))

    # scale data to values more suitable for visualization
    mel = np.sum(mel, axis=0)
    mel = mel ** 2.0
    print("mel", mel.shape, mel[:5])

    # gain normalization
    # mel_gain.update(np.max(gaussian_filter1d(mel, sigma=1.0)))
    gaussian_mel = gaussian_filter1d(mel, sigma=1.0)
    print("gaussian mel", gaussian_mel.shape, mel[:5])
    print(
        "gaussian mel ( min %f max %f )" % (np.min(gaussian_mel), np.max(gaussian_mel))
    )

    gain_update = np.max(gaussian_mel)
    print("max mel", gain_update)
    mel_gain_val = mel_gain.update(gain_update)

    print("mel gain value", mel_gain_val.shape, mel_gain_val)
    mel /= mel_gain_val
    print("mel after gain norm", mel.shape, mel)
    print(
        "( min %f max %f )" % (np.min(mel), np.max(mel))
    )

    mel = mel_smoothing.update(mel)
    print("mel after smoothing", mel.shape, mel)
    print(
        "( min %f max %f )" % (np.min(mel), np.max(mel))
    )

    # print("samples", samples)
