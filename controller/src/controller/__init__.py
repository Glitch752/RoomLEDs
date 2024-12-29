import http.server
import _thread as thread
import time
import serial

TOTAL_PIXELS = 814
TARGET_FPS = 60

DRIVER_1_SERIAL_PORT = "/dev/ttyUSB0"
# DRIVER_2_SERIAL_PORT = "/dev/ttyUSB1"

# 1M baud is the absolute highest speed we can push the ESP8266 to
driver_1 = serial.Serial(DRIVER_1_SERIAL_PORT, 1_000_000)
# driver_2 = serial.Serial(DRIVER_2_SERIAL_PORT, 1_000_000)

def hsv_to_rgb(h: float, s: float, v: float) -> tuple[float, float, float]:
    if s:
        if h == 1.0: h = 0.0
        i = int(h*6.0); f = h*6.0 - i
        
        w = v * (1.0 - s)
        q = v * (1.0 - s * f)
        t = v * (1.0 - s * (1.0 - f))
        
        if i==0: return (v, t, w)
        if i==1: return (q, v, w)
        if i==2: return (w, v, t)
        if i==3: return (w, q, v)
        if i==4: return (t, w, v)
        if i==5: return (v, w, q)
    else: return (v, v, v)

class PixelDataPointer:
    pixel_data: "PixelData"
    index: int
    
    def __init__(self, pixel_data: "PixelData", index: int):
        self.pixel_data = pixel_data
        self.index = index
    
    def set_from_hsv(self, h: float, s: float, v: float):
        rgb = hsv_to_rgb(h / 255, s / 255, v / 255)
        self.pixel_data.data[self.index * 3 : self.index * 3 + 3] = bytearray([int(c * 255) for c in rgb])
    def set_from_rgb(self, r: int, g: int, b: int):
        self.pixel_data.data[self.index * 3 : self.index * 3 + 3] = bytearray([r, g, b])
    
    def __getitem__(self, index: int) -> int:
        return self.pixel_data.data[self.index * 3 + index]
    
    def __setitem__(self, index: int, value: int):
        self.pixel_data.data[self.index * 3 + index] = value
    
    def __iter__(self):
        return iter([self[0], self[1], self[2]])

class PixelData:
    total_pixels: int
    data: bytearray
    pointers: list[PixelDataPointer] # Cache pointers to avoid creating thousands of objects
    
    def __init__(self, total_pixels: int):
        self.total_pixels = total_pixels
        self.data = bytearray([0, 0, 0] * total_pixels)
        self.pointers = [PixelDataPointer(self, i) for i in range(total_pixels)]
    
    def __getitem__(self, index: int) -> PixelDataPointer:
        return self.pointers[index]

    def __setitem__(self, index: int, value: PixelDataPointer):
        self.data[index * 3 : index * 3 + 3] = bytearray(value)

PIXEL_DATA = PixelData(TOTAL_PIXELS)

last_update_time = time.time()
def animate():
    global start_hue, last_update_time
    
    while True:
        render()
        send_pixel_data()

def send_pixel_data():
    # TODO
    # This should probably be on another thread so we can guarentee we'll send pixel data at the right time
    
    # Driver 1 gets the first half of the pixel data
    driver_1_data = PIXEL_DATA.data[:len(PIXEL_DATA.data) // 2]
    # driver_2_data = PIXEL_DATA.data[len(PIXEL_DATA.data) // 2:]
    
    # TODO: A simple checksum?
    
    # Wait for a '>' character indicating we can start sending data
    while driver_1.read() != b">":
        pass    
    
    time.sleep(0.01) # Wait a bit to make sure the driver is ready
    
    # Send a '<' character to indicate the start of the pixel data
    driver_1.write(b"<")
    
    # Send the pixel data
    driver_1.write(driver_1_data)

def display_pixel_data_on_console():
    # Move the cursor to the start of the line and overwrite the previous frame
    line = "\033[0G"
    
    # Temorary: print the pixel data to the console using ANSI full-color escape codes
    for i in range(min(TOTAL_PIXELS, 100)):
        r, g, b = PIXEL_DATA[i]
        line += f"\033[38;2;{r};{g};{b}m█"
    
    line += "\033[0m" # Reset color to default
    
    print(line, end="", flush=True) # Print without newline and flush the buffer to make sure it's printed immediately

start_hue = 0
def render():
    global start_hue
    import random
    
    start_hue += 4
    if start_hue > 255:
        start_hue = 0

    for i in range(TOTAL_PIXELS):
        hue = (start_hue + i * 4) % 255
        # if random.randint(0, 100) < 5:
        #     PIXEL_DATA[i].set_from_rgb(255, 255, 255)
        # else:
        PIXEL_DATA[i].set_from_hsv(hue, 255, 255)
        
        # PIXEL_DATA[i].set_from_rgb(random.randint(0, 255), random.randint(0, 255), random.randint(0, 255))

class MyHandler(http.server.SimpleHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers["Content-Length"])
        post_data = self.rfile.read(content_length)
        print(post_data.decode("utf-8"))
        
        self.send_response(200)
        self.end_headers()
    
    def do_GET(self):
        self.send_response(200)
        self.headers["Content-type"] = "text/html"
        self.end_headers()
        
        str = f"<html><head><title>Pixel Controller</title></head><body><h1>Pixel Controller</h1><p>Pixel count: {TOTAL_PIXELS}</p></body></html>"
        self.wfile.write(str.encode("utf-8"))

def start_server():
    server_address = ("", 5000)
    httpd = http.server.HTTPServer(server_address, MyHandler)
    print("Starting server on port 5000")
    
    httpd.serve_forever()

def main() -> int:
    thread.start_new_thread(start_server, ())
    
    animate()
    
    return 0