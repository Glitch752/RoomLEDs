import http.server

class MyHandler(http.server.SimpleHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers["Content-Length"])
        post_data = self.rfile.read(content_length)
        print(post_data.decode("utf-8"))
        self.send_response(200)
        self.end_headers()
    
    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"Hello, world!")

def main() -> int:
    server_address = ("", 5000)
    httpd = http.server.HTTPServer(server_address, MyHandler)
    print("Starting server on port 5000")
    
    httpd.serve_forever()
    
    return 0