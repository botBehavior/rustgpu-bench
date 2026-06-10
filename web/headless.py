#!/usr/bin/env python3
"""Headless WebGPU verifier.

Serves gpu/web over HTTP, opens a `?auto` page in headless Chrome (real GPU via
Vulkan), and captures the page's /AUTO/<msg> beacons. The page is expected to
render a burst of frames headless and beacon a health line + a *-DONE marker.

Usage:  python headless.py [page.html] [seconds]
Exit 0 if a healthy *-DONE beacon arrives with no ERROR; 1 otherwise.
Reusable for any ?auto page (mycelium.html, mycelia.html, ...).
"""
import http.server
import socketserver
import threading
import subprocess
import sys
import os
import time
import socket
import urllib.parse

WEB = os.path.dirname(os.path.abspath(__file__))
PAGE = sys.argv[1] if len(sys.argv) > 1 else "mycelium.html"
DUR = float(sys.argv[2]) if len(sys.argv) > 2 else 30.0
beacons = []


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *a, **k):
        super().__init__(*a, directory=WEB, **k)

    def do_GET(self):
        if self.path.startswith("/AUTO/"):
            msg = urllib.parse.unquote(self.path[6:])
            beacons.append(msg)
            print("  beacon:", msg, flush=True)
            self.send_response(200)
            self.send_header("Content-Length", "2")
            self.end_headers()
            self.wfile.write(b"ok")
            return
        return super().do_GET()

    def log_message(self, *a):
        pass


def free_port():
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    p = s.getsockname()[1]
    s.close()
    return p


def main():
    port = free_port()
    srv = socketserver.ThreadingTCPServer(("127.0.0.1", port), Handler)
    srv.daemon_threads = True
    threading.Thread(target=srv.serve_forever, daemon=True).start()
    url = f"http://127.0.0.1:{port}/{PAGE}?auto"
    print(f"serving {WEB} on :{port}  ->  {url}", flush=True)

    chrome = None
    for c in (r"C:\Program Files\Google\Chrome\Application\chrome.exe",
              r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"):
        if os.path.exists(c):
            chrome = c
            break
    if not chrome:
        print("no Chrome/Edge found")
        return 2

    prof = os.path.join(os.environ.get("TEMP", "."), "cgpu_headless")
    shot = os.path.join(WEB, "_shot.png")
    args = [
        chrome, "--headless=new", "--enable-unsafe-webgpu",
        "--enable-features=Vulkan", "--no-sandbox", "--disable-gpu-sandbox",
        f"--user-data-dir={prof}", "--window-size=960,600",
        f"--screenshot={shot}", "--virtual-time-budget=22000", url,
    ]
    print("launch:", os.path.basename(chrome), flush=True)
    proc = subprocess.Popen(args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    deadline = time.time() + DUR
    while time.time() < deadline:
        if any(b.endswith("-DONE") or "ERROR" in b for b in beacons):
            break
        time.sleep(0.3)
    time.sleep(0.5)
    try:
        proc.terminate()
        proc.wait(timeout=5)
    except Exception:
        proc.kill()
    srv.shutdown()

    ok = any(b.endswith("-DONE") for b in beacons) and not any("ERROR" in b for b in beacons)
    print("RESULT:", "OK" if ok else "FAIL", flush=True)
    for b in beacons:
        print("  ", b)
    print("screenshot:", shot if os.path.exists(shot) else "(none)")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
