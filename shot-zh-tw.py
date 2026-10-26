#!/usr/bin/env python3
"""Drive a running cadcraft (--control PORT) to capture zh-TW screenshots.
Usage: shot-zh-tw.py <port> <outdir>
Assumes the app was started with: cadcraft --sample --control <port>
Reads the per-session token from ~/.cache/cadcraft-control-<port>.token
"""
import json, socket, sys, time, os

port = int(sys.argv[1])
outdir = sys.argv[2]
os.makedirs(outdir, exist_ok=True)

tokpath = os.path.expanduser(f"~/.cache/cadcraft-control-{port}.token")
for _ in range(30):
    if os.path.exists(tokpath):
        break
    time.sleep(1)
token = open(tokpath).read().strip()
print("token file:", tokpath, "len:", len(token))

def call(sock, rid, method, params=None):
    req = {"id": rid, "method": method, "token": token}
    if params is not None:
        req["params"] = params
    sock.sendall((json.dumps(req) + "\n").encode())
    buf = b""
    while b"\n" not in buf:
        chunk = sock.recv(65536)
        if not chunk:
            break
        buf += chunk
    line = buf.split(b"\n")[0].decode()
    try:
        return json.loads(line)
    except Exception:
        return {"raw": line[:300]}

s = socket.create_connection(("127.0.0.1", port), timeout=30)
rid = 1

r = call(s, rid, "engine.execute",
         {"command": "ui.language", "params": {"lang": "zh-TW"}})
print("ui.language:", json.dumps(r)[:200]); rid += 1
time.sleep(3)

main_png = os.path.join(outdir, "shot-main-zh-tw.png")
r = call(s, rid, "ui.screenshot", {"path": main_png})
print("screenshot main:", json.dumps(r)[:200]); rid += 1
time.sleep(1)

r = call(s, rid, "ui.menu.invoke", {"command": "ui.dialog.about"})
print("menu.invoke about:", json.dumps(r)[:200]); rid += 1
time.sleep(3)

about_png = os.path.join(outdir, "shot-about-zh-tw.png")
r = call(s, rid, "ui.screenshot", {"path": about_png})
print("screenshot about:", json.dumps(r)[:200]); rid += 1

s.close()
for p in (main_png, about_png):
    print(p, os.path.getsize(p) if os.path.exists(p) else "MISSING")
