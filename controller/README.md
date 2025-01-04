The controller for my RoomLEDs project. This is a Python script that runs on a single board computer (currently the Libre Computer Le Potato) that sends serial data to ESP8266 microcontrollers connected to WS2812B LED strips. The script is set up to run as a systemd service at boot.

To use `deploy.sh`, set `$SERVER_USER` and `$SERVER_IP`, then make sure you have passwordless login through SSH set up on the target machine. You can do this with the following commands:
```bash
ssh-keygen # Press enter to accept the default location (or change it) and no passphrase
ssh-copy-id -i ~/.ssh/id_ed25519.pub $SERVER_USER@$SERVER_IP # Or whatever your key is named
```
Then, set `$SERVER_IDENTITY_FILE` to the path of the private key you just generated (`~/.ssh/id_ed25519` by default).
You may also optionally set `$PASSWORD` to the password for the target machine; otherwise, you'll be prompted for it when running `deploy.sh`.

This is probably really insecure and bad, but I don't particularly care about the security of my lighting system for now. Hopefully I can come up with a better solution in the future.

## TODO
- [ ] Make the serial communication much more robust
- [X] Make a simple web interface for controlling the lights
- [X] Rewrite in Rust or some other language with stronger real-time guarantees