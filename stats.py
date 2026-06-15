import sys
from collections import deque

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from matplotlib.animation import PillowWriter

# main configuration
DATA_LENGTH = int(input())
FPS = 60
SAMPLE_RATE = 100
BIN_COUNT = 100
TEST_NAME = sys.argv[1]
INIT_DELAY_SECS = 1
END_DELAY_SECS = 10

class LiveHistogram:
    def __init__(self, 
        test_name: str,
        data_length: int,
        sample_rate: int, 
        bin_count: int, 
        fps: int, 
        init_delay_secs: int, 
        end_delay_secs: int, 
    ) -> None:
        self.fps = fps
        self.sample_rate = sample_rate
        self.bin_count = bin_count
        self.test_name = test_name
        self.data = deque()

        # i do this math to make this sample rate agnostic
        self.start_delay_frame_count = self.fps * init_delay_secs
        self.end_delay_frame_count = self.fps * end_delay_secs
        self.frames = data_length // self.sample_rate

        fig, self.axes = plt.subplots(figsize=(10, 5)) # resize window
        fig.tight_layout(pad=2.0) # autoscale layout with padding
        self.animation_fn = FuncAnimation(
            fig,
            self.update,
            frames=self.frames + self.start_delay_frame_count + self.end_delay_frame_count,
            cache_frame_data=False,
            interval=1000//self.fps
        )

    def save_as_gif(self):
        # saves a gif by default
        self.animation_fn.save(
            f'{self.test_name}.gif', 
            writer=PillowWriter(fps=self.fps)
        )
        
    def update(self, frame):
        if frame <= self.start_delay_frame_count:
            # show test name even on blank graph
            self.axes.set_title(f'test={self.test_name}')
            return
        if frame > self.frames + self.start_delay_frame_count:
            return
        for _ in range(self.sample_rate):
            # if DATA_LENGTH % SAMPLE_RATE > 0 this will EOFError
            self.data.append(float(input()))
        # clear graph and redraw with constant settings
        self.axes.clear()
        self.axes.margins(x=0.05, y=0.05) # give x and y margins to the histogram
        self.axes.set_xlabel("Latency (us)")
        self.axes.set_ylabel("Frequency")

        # compute data and redraw histogram from it
        self.axes.set_title(
            f"test={self.test_name} | "
            f"mean={np.mean(self.data):.2f} | "
            f"p50={np.percentile(self.data, 50):.2f} | "
            f"p99={np.percentile(self.data, 99):.2f} | "
            f"std={np.std(self.data):.2f} | "
            f"n_samples={len(self.data)}"
        )
        # evenly space, everything else is autoscale
        bins = np.linspace(min(self.data), max(self.data), self.bin_count+1)
        plt.hist(self.data, bins=bins, edgecolor="black")
            
if __name__ == "__main__":
    live = LiveHistogram(
        TEST_NAME,
        DATA_LENGTH,
        SAMPLE_RATE, 
        BIN_COUNT, 
        FPS, 
        INIT_DELAY_SECS, 
        END_DELAY_SECS, 
    )
    live.save_as_gif()
