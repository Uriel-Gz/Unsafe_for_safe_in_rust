#!/usr/bin/env python3
import zlib, os

MAP = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"

def encode6bit(b):
    return MAP[b]

def plantuml_encode(s: str) -> str:
    data = zlib.compress(s.encode('utf-8'))
    data = data[2:-4]
    res = []
    i = 0
    while i < len(data):
        b1 = data[i]
        b2 = data[i+1] if i+1 < len(data) else 0
        b3 = data[i+2] if i+2 < len(data) else 0
        c1 = (b1 >> 2) & 0x3F
        c2 = ((b1 & 0x3) << 4) | ((b2 >> 4) & 0xF)
        c3 = ((b2 & 0xF) << 2) | ((b3 >> 6) & 0x3)
        c4 = b3 & 0x3F
        res.append(encode6bit(c1))
        res.append(encode6bit(c2))
        res.append(encode6bit(c3))
        res.append(encode6bit(c4))
        i += 3
    return ''.join(res)

root = os.path.join(r"c:\UNRC\5to_año\_Tesis_","Documentacion","diagrams")
files = ["activity_pipeline.puml","sequence_main.puml","components_src.puml"]

from urllib.request import urlopen, Request
from urllib.error import URLError, HTTPError
import sys

def download_plantuml_png(puml_path: str, out_path: str) -> bool:
    try:
        with open(puml_path, 'r', encoding='utf-8') as fh:
            txt = fh.read()
    except Exception as e:
        print(f"Error reading {puml_path}: {e}", file=sys.stderr)
        return False

    enc = plantuml_encode(txt)
    url = f"https://www.plantuml.com/plantuml/png/{enc}"
    try:
        req = Request(url, headers={"User-Agent": "plantuml-client"})
        with urlopen(req, timeout=30) as resp:
            data = resp.read()
        with open(out_path, 'wb') as out:
            out.write(data)
        print(f"Saved: {out_path}")
        print(f"URL: {url}")
        return True
    except HTTPError as e:
        print(f"HTTP error {e.code} downloading {url}: {e}", file=sys.stderr)
    except URLError as e:
        print(f"URL error {e.reason} downloading {url}", file=sys.stderr)
    except Exception as e:
        print(f"Unexpected error downloading {url}: {e}", file=sys.stderr)
    return False


if __name__ == '__main__':
    any_ok = False
    for f in files:
        puml = os.path.join(root, f)
        out_png = os.path.join(root, os.path.splitext(f)[0] + '.png')
        ok = download_plantuml_png(puml, out_png)
        any_ok = any_ok or ok
    if not any_ok:
        print("No images were downloaded. Check network or PlantUML server.", file=sys.stderr)
