# CyberLED
ESP32 hack project

## Setup
In no particular order...
```
7z x .\CP210x_Universal_Windows_Driver.zip
cargo install espup
espup install -t esp32
cargo install espflash
cargo install esp-generate --locked
esp-generate.exe --chip esp32 cyberled
```