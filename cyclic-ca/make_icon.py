#!/usr/bin/env python3
"""Generate a macOS .icns icon from a cyclic cellular automata snapshot."""

import random
import math
import os
import shutil
from PIL import Image

# --- CA Simulation ---

NUM_TYPES = 16
WIDTH = 512
HEIGHT = 512
STEPS = 80

def rainbow_color(i, num_types):
    t = i / num_types
    h = t * 6.0
    c = 1.0
    x = c * (1.0 - abs((h % 2.0) - 1.0))
    if   h < 1: r, g, b = c, x, 0.0
    elif h < 2: r, g, b = x, c, 0.0
    elif h < 3: r, g, b = 0.0, c, x
    elif h < 4: r, g, b = 0.0, x, c
    elif h < 5: r, g, b = x, 0.0, c
    else:        r, g, b = c, 0.0, x
    return (int(r * 255), int(g * 255), int(b * 255))

colors = [rainbow_color(i, NUM_TYPES) for i in range(NUM_TYPES)]

# Seed with a center blob pattern for icon appeal
rng = random.Random(42)
grid = [[rng.randrange(NUM_TYPES) for _ in range(WIDTH)] for _ in range(HEIGHT)]

# Bias center toward low types to seed spiral growth
cx, cy = WIDTH // 2, HEIGHT // 2
for y in range(HEIGHT):
    for x in range(WIDTH):
        dx, dy = x - cx, y - cy
        dist = math.sqrt(dx*dx + dy*dy)
        if dist < WIDTH * 0.15:
            angle = math.atan2(dy, dx)
            t = (angle + math.pi) / (2 * math.pi)
            grid[y][x] = int(t * NUM_TYPES) % NUM_TYPES

def step(grid):
    next_g = [row[:] for row in grid]
    for y in range(HEIGHT):
        for x in range(WIDTH):
            cur = grid[y][x]
            prey = (cur - 1) % NUM_TYPES
            neighbors = [
                grid[(y-1) % HEIGHT][x],
                grid[(y+1) % HEIGHT][x],
                grid[y][(x-1) % WIDTH],
                grid[y][(x+1) % WIDTH],
                grid[(y-1) % HEIGHT][(x-1) % WIDTH],
                grid[(y-1) % HEIGHT][(x+1) % WIDTH],
                grid[(y+1) % HEIGHT][(x-1) % WIDTH],
                grid[(y+1) % HEIGHT][(x+1) % WIDTH],
            ]
            if any(n == prey for n in neighbors):
                next_g[y][x] = prey
    return next_g

print(f"Simulating {STEPS} steps at {WIDTH}x{HEIGHT}...")
for i in range(STEPS):
    if i % 20 == 0:
        print(f"  step {i}/{STEPS}")
    grid = step(grid)

print("Rendering image...")
img = Image.new("RGB", (WIDTH, HEIGHT))
pixels = img.load()
for y in range(HEIGHT):
    for x in range(WIDTH):
        pixels[x, y] = colors[grid[y][x]]

# --- Build iconset ---
ICONSET = "/tmp/CyclicCA.iconset"
os.makedirs(ICONSET, exist_ok=True)

sizes = [16, 32, 64, 128, 256, 512, 1024]
filenames = {
    16:   "icon_16x16.png",
    32:   "icon_16x16@2x.png",
    64:   "icon_32x32@2x.png",
    128:  "icon_128x128.png",
    256:  "icon_128x128@2x.png",
    512:  "icon_256x256@2x.png",
    1024: "icon_512x512@2x.png",
}
# also need non-retina for 32, 256, 512
extra = {
    32:  "icon_32x32.png",
    256: "icon_256x256.png",
    512: "icon_512x512.png",
}

for size, fname in {**filenames, **extra}.items():
    resized = img.resize((size, size), Image.LANCZOS)
    resized.save(os.path.join(ICONSET, fname))
    print(f"  saved {fname}")

print("Packaging .icns...")
os.system(f"iconutil -c icns {ICONSET} -o /tmp/CyclicCA.icns")
print("Done: /tmp/CyclicCA.icns")
