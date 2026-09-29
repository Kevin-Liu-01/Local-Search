#!/usr/bin/env python3
"""Encode authored HTML frames as GIF and MP4. Requires Pillow and ffmpeg."""
from pathlib import Path
import json
import subprocess
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'site/public/social/2026-09'
FPS = 20
MOVIES = {
    'agent-browser': [('hero', 2.0), ('search', 2.5), ('results', 3.5), ('read', 4.5), ('outro', 3.5)],
    'browser-choice': [('choice', 4.0), ('connect', 4.5), ('disconnect', 3.5)],
}


def render(name, shots):
    """Crossfade scene transitions, then hold readable frames. No speed simulation."""
    proc = subprocess.Popen([
        'ffmpeg', '-v', 'error', '-y', '-f', 'rawvideo', '-pixel_format', 'rgb24',
        '-video_size', '1200x900', '-framerate', str(FPS), '-i', '-', '-an',
        '-c:v', 'libx264', '-crf', '18', '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
        str(OUT / f'{name}.mp4'),
    ], stdin=subprocess.PIPE)
    previous = None
    for scene, seconds in shots:
        frame = Image.open(OUT / f'{scene}.png').convert('RGB')
        for i in range(round(seconds * FPS)):
            current = Image.blend(previous, frame, (i + 1) / 6) if previous is not None and i < 6 else frame
            proc.stdin.write(current.tobytes())
        previous = frame
    proc.stdin.close()
    if proc.wait() != 0:
        raise RuntimeError('Video encoding failed')
    subprocess.run([
        'ffmpeg', '-v', 'error', '-y', '-i', str(OUT / f'{name}.mp4'),
        '-filter_complex', '[0:v]fps=10,scale=960:720:flags=lanczos,split[a][b];[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=bayer:bayer_scale=4',
        '-loop', '0', str(OUT / f'{name}.gif'),
    ], check=True)
    with Image.open(OUT / f'{name}.gif') as gif:
        count = gif.n_frames
        duration = 0
        for i in range(count):
            gif.seek(i)
            gif.load()
            duration += gif.info.get('duration', 0)
        assert gif.size == (960, 720)
        assert abs(duration / 1000 - sum(s for _, s in shots)) < .2
    return dict(name=name, frames=count, seconds=duration/1000,
                gif_bytes=(OUT/f'{name}.gif').stat().st_size,
                mp4_bytes=(OUT/f'{name}.mp4').stat().st_size)


if __name__ == '__main__':
    data = [render(name, shots) for name, shots in MOVIES.items()]
    (OUT/'encoding-checks.json').write_text(json.dumps(data, indent=2)+'\n')
    print(json.dumps(data, indent=2))
