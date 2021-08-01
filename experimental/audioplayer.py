import sys
import time
import numpy as np
import pyaudio
import wave
import melbank
import tkinter as tk
import threading
from SimpleWebSocketServer import SimpleWebSocketServer, WebSocket
from scipy.ndimage.filters import gaussian_filter1d
from audioanalyzer import AudioProcessor

root = tk.Tk()

# async def hello(websocket, path):
#     name = await websocket.recv()
#     print(f"< {name}")

#     greeting = f"Hello {name}!"

#     await websocket.send(greeting)
#     print(f"> {greeting}")


class SimpleEcho(WebSocket):

    # def broadcast(self, value):
    #    # if self.data is None:
    #        # self.data = ''

    #    for client in self.server.connections.itervalues():
    #        # client.sendMessage(str(self.address[0]) + ' - ' + str(self.data))
    #        client.sendMessage(value)

    #    #echo message back to client
    #    #self.sendMessage(str(self.data))

    def handleConnected(self):
        print(self.address, "connected")

    def handleClose(self):
        print(self.address, "closed")


server = SimpleWebSocketServer("", 9000, SimpleEcho)


class AudioAnalyzer(threading.Thread):
    def run(self):
        stream_from_file(sys.argv[1])
        root.quit()
        root.update()


class Webserver(threading.Thread):
    def run(self):
        server.serveforever()
        # start_server = websockets.serve(hello, "localhost", 8765)
        # asyncio.get_event_loop().run_until_complete(start_server)
        # asyncio.get_event_loop().run_forever()


webserver = Webserver()
analyzer = AudioAnalyzer()

# MIC_SAMPLE_RATE = 44100  # 48000
# def start_stream(callback):
#     p = pyaudio.PyAudio()
#     frames_per_buffer = int(MIC_SAMPLE_RATE / config.FPS)
#     stream = p.open(format=pyaudio.paInt16,
#                     channels=1,
#                     rate=config.MIC_RATE,
#                     input=True,
#                     frames_per_buffer=frames_per_buffer)
#     overflows = 0
#     prev_ovf_time = time.time()
#     while True:
#         try:
#             y = np.fromstring(stream.read(frames_per_buffer, exception_on_overflow=False), dtype=np.int16)
#             y = y.astype(np.float32)
#             stream.read(stream.get_read_available(), exception_on_overflow=False)
#             callback(y)
#         except IOError:
#             overflows += 1
#             if time.time() > prev_ovf_time + 1:
#                 prev_ovf_time = time.time()
#                 print('Audio buffer has overflowed {} times'.format(overflows))
#     stream.stop_stream()
#     stream.close()
#     p.terminate()


# class ExpFilter:
#     """Simple exponential smoothing filter"""

#     def __init__(self, val=0.0, alpha_decay=0.5, alpha_rise=0.5):
#         """Small rise / decay factors = more smoothing"""
#         assert 0.0 < alpha_decay < 1.0, "Invalid decay smoothing factor"
#         assert 0.0 < alpha_rise < 1.0, "Invalid rise smoothing factor"
#         self.alpha_decay = alpha_decay
#         self.alpha_rise = alpha_rise
#         self.value = val

#     def update(self, value):
#         if isinstance(self.value, (list, np.ndarray, tuple)):
#             alpha = value - self.value
#             alpha[alpha > 0.0] = self.alpha_rise
#             alpha[alpha <= 0.0] = self.alpha_decay
#         else:
#             alpha = self.alpha_rise if value > self.value else self.alpha_decay
#         self.value = alpha * value + (1.0 - alpha) * self.value
#         return self.value


# MIN_VOLUME_THRESHOLD = 1e-7
# N_FRAMES_ROLLING_WINDOW = 2

# N_FFT_BINS = 24
# MIN_FREQUENCY = 200
# MAX_FREQUENCY = 12000

# mel_gain = ExpFilter(np.tile(1e-1, N_FFT_BINS), alpha_decay=0.01, alpha_rise=0.99)
# mel_smoothing = ExpFilter(np.tile(1e-1, N_FFT_BINS), alpha_decay=0.5, alpha_rise=0.99)
# gain = ExpFilter(np.tile(0.01, N_FFT_BINS), alpha_decay=0.001, alpha_rise=0.99)


# class AudioProcessorOld:
#     def __init__(self, sample_rate, fps, nchannels):
#         self.sample_rate = sample_rate
#         self.fps = fps
#         self.nchannels = nchannels

#         self.samples_per_frame = int(self.sample_rate / self.fps)

#         self.fft_window = np.hamming(self.samples_per_frame * N_FRAMES_ROLLING_WINDOW)

#         self.roll_win = (
#             np.zeros((N_FRAMES_ROLLING_WINDOW, self.samples_per_frame)) / 1e16
#         )

#         mel_samples = int(self.sample_rate * N_FRAMES_ROLLING_WINDOW / (2.0 * self.fps))
#         self.mel_y, (_, self.mel_x) = melbank.compute_melmat(
#             num_mel_bands=N_FFT_BINS,
#             freq_min=MIN_FREQUENCY,
#             freq_max=MAX_FREQUENCY,
#             num_fft_bands=mel_samples,
#             sample_rate=self.sample_rate,
#         )

#         self.visualization_effect = self.visualize_scroll

#     def visualize_scroll(self, y):
#         # print(y)
#         # print(y.shape)
#         y = y ** 2.0
#         gain.update(y)
#         y /= gain.value
#         y *= 255.0
#         r = int(np.max(y[: len(y) // 3]))
#         g = int(np.max(y[len(y) // 3 : 2 * len(y) // 3]))
#         b = int(np.max(y[2 * len(y) // 3 :]))
#         print(r, g, b)
#         return (r, g, b)
#         # Scrolling effect window
#         p[:, 1:] = p[:, :-1]
#         p *= 0.98
#         p = gaussian_filter1d(p, sigma=0.2)
#         # Create new color originating at the center
#         p[0, 0] = r
#         p[1, 0] = g
#         p[2, 0] = b
#         # Update the LED strip
#         return np.concatenate((p[:, ::-1], p), axis=1)

#     def process(self, stereo):
#         # combine stereo signal to mono
#         mono = np.amax(stereo, axis=0)
#         if mono.shape[0] != self.samples_per_frame:
#             return
#         # normalize
#         mono = mono / 2 ** 15

#         # add to rolling window
#         self.roll_win[:-1] = self.roll_win[1:]
#         self.roll_win[-1, :] = np.copy(mono)
#         window = np.concatenate(self.roll_win, axis=0).astype(np.float32)

#         volume = np.max(np.abs(mono))
#         if volume < MIN_VOLUME_THRESHOLD:
#             # ignore for now
#             pass
#         else:
#             window_size = len(window)

#             # pad with zeros until the next power of two
#             next_power_of_two = int(np.ceil(np.log2(window_size)))
#             padding = 2 ** next_power_of_two - window_size

#             window *= self.fft_window
#             window = np.pad(window, (0, padding), mode="constant")
#             ys = np.abs(np.fft.rfft(window)[: window_size // 2])

#             # construct a Mel filterbank from the FFT data
#             mel = np.atleast_2d(ys).T * self.mel_y.T

#             # scale data to values more suitable for visualization
#             mel = np.sum(mel, axis=0)
#             mel = mel ** 2.0

#             # gain normalization
#             mel_gain.update(np.max(gaussian_filter1d(mel, sigma=1.0)))
#             mel /= mel_gain.value
#             mel = mel_smoothing.update(mel)

#             rgb = self.visualization_effect(mel)
#             color = rgb_to_hex(*rgb)
#             root.configure(bg=color)
#             for c in server.connections.values():
#                 c.sendMessage("test")


def clip(x, lower, upper):
    return max(min(x, upper), lower)


def rgb_to_hex(r, g, b):
    return "#%02x%02x%02x" % tuple([clip(comp, 0, 255) for comp in [r, g, b]])


def stream_from_file(path):
    wf = wave.open(path, "rb")
    p = pyaudio.PyAudio()

    sample_rate = wf.getframerate()
    nchannels = wf.getnchannels()
    nbytesframe = wf.getsampwidth()
    audio_format = p.get_format_from_width(wf.getsampwidth())

    fps = 60
    frames_per_buffer = int(sample_rate / fps)

    processor = AudioProcessor(sample_rate, fps, nchannels)

    def callback(in_data, frame_count, time_info, status):
        raw_data = wf.readframes(frame_count)
        data = np.frombuffer(
            raw_data, dtype=(np.uint8 if nbytesframe == 1 else np.int16)
        )
        # if nchannels == 1:
        #     data = np.array([data, data])
        # else:
        #     data = np.array([data[::2], data[1::2]])
        rgb = processor.process(data)
        # print(rgb)
        if rgb is not None:
            # rgb can be none when there is silence
            color = rgb_to_hex(*rgb)
            root.configure(bg=color)
            for c in server.connections.values():
                c.sendMessage("test")

        return (raw_data, pyaudio.paContinue)

    stream = p.open(
        format=audio_format,
        channels=nchannels,
        rate=sample_rate,
        frames_per_buffer=frames_per_buffer,
        stream_callback=callback,
        output=True,
    )

    stream.start_stream()
    while stream.is_active():
        time.sleep(0.1)

    stream.stop_stream()
    stream.close()
    wf.close()
    p.terminate()


if __name__ == "__main__":
    analyzer.start()

    webserver.start()

    root.title("preview")
    root.geometry("200x200")
    root.mainloop()
