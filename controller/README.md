The controller for my RoomLEDs project. This is a Python script that runs on a single board computer (currently the Libre Computer Le Potato) that sends serial data to ESP8266 microcontrollers connected to WS2812B LED strips. The script is set up to run as a systemd service at boot.

To use `deploy.sh`, set `$SERVER_USER` and `$SERVER_IP`, then make sure you have passwordless login through SSH set up on the target machine. You can do this with the following commands:
```bash
ssh-keygen # Press enter to accept the default location (or change it) and no passphrase
ssh-copy-id -i ~/.ssh/id_ed25519.pub $SERVER_USER@$SERVER_IP # Or whatever your key is named
```
Then, set `$SERVER_IDENTITY_FILE` to the path of the private key you just generated (`~/.ssh/id_ed25519` by default).
You may also optionally set `$PASSWORD` to the password for the target machine; otherwise, you'll be prompted for it when running `deploy.sh`.

This is probably really insecure and bad, but I don't particularly care about the security of my lighting system for now. Hopefully I can come up with a better solution in the future.

## Setting up mDNS
This is mostly for my own reference, but here's how you can set up multicast DNS on Debian to make it easier to access the controller. This is useful for accessing the web interface without needing to know the IP address of the machine.  

```bash
sudo apt install avahi-daemon avahi-utils nginx # Install the avahi daemon, avahi utils, and nginx
```

Create a new nginx configuration file:
```bash
sudo nano /etc/nginx/sites-available/lights
```

Add the following configuration:
```nginx
server {
    listen 80;
    server_name lights.local; # Make sure this matches the name you want to use

    # Normal HTTP traffic
    location / {
        proxy_pass http://localhost:3000;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # WebSockets
    location /websocket {
        proxy_pass http://localhost:3000;

        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

Enable the new site:
```bash
sudo ln -s /etc/nginx/sites-available/lights /etc/nginx/sites-enabled/
sudo nginx -t # Test the configuration; good practice to do this before restarting nginx
sudo systemctl restart nginx
```

Now, you should be able to access the web interface at `http://hostname.local` (or whatever the hostname of your device is). If you wish to change the hostname of your device, you can edit `/etc/hostname`, then restart.

### Using a different hostname
If you want to make the mDNS address something other than your hostname, Avahi will complain because it only allows multiple mDNS addresses to map to a single IP if you run `avahi-publish` with the `-R` flag. The easiest way to do this is to create a new service file:
```bash
sudo nano /etc/systemd/system/avahi-publish-lights.service
```
Add the following configuration:
```ini
[Unit]
Description=Avahi publish lights.local
After=avahi-daemon.service
Requires=avahi-daemon.service
[Service]
Type=simple
ExecStart=/usr/bin/avahi-publish -s lights.local _http._tcp 80 "Lights" "RoomLEDs server"
[Install]
WantedBy=multi-user.target
```
Then, enable and start the service:
```bash
sudo systemctl enable avahi-publish-lights
sudo systemctl start avahi-publish-lights
```

## TODO
- [X] Make the serial communication much more robust
- [X] Make a simple web interface for controlling the lights
- [X] Rewrite in Rust or some other language with stronger real-time guarantees