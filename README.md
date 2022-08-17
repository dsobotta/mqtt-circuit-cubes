# mqtt-circuit-cubes
BLE interface and MQTT bridge support for Tenka Circuit Cubes.

Supported platforms:
- [x] x64 Linux
- [x] arm32 Linux
- [x] arm64 Linux
- [ ] riscv64 Linux
- [ ] esp32 (IDF)
- [ ] esp32-c3 (IDF)

APIs:
- [x] Python
- [ ] MicroPython
- [ ] C/C++
- [ ] Rust

# Installation
```
  #Debian-based distros
  sudo apt update
  sudo apt upgrade
  sudo apt install git python pip mosquitto mosquitto-clients
  
  #Arch-based distros
  sudo pacman -Syuu
  sudo pacman -S git python python-pip mosquitto
  
  #mqtt-circuit-cube sources
  git clone https://github.com/dsobotta/mqtt-circuit-cubes.git
  cd mqtt-circuit-cubes/python
  pip install .
  
```

# Usage
```
  chmod +x bin/*
  bin/mqtt-cc-bridge
  
  #in a separate terminal session...
  mosquitto_pub -h localhost -t "<CC_DEVICE_ID>/cmd/power/a" -m "255" #output a full power
  mosquitto_pub -h localhost -t "<CC_DEVICE_ID>/cmd/power/b" -m "-255" #output b full reverse power
  mosquitto_pub -h localhost -t "<CC_DEVICE_ID>/cmd/power/c" -m "0" #output c power off
  mosquitto_sub -h localhost -t "<CC_DEVICE_ID>/status/battery" #subscribe to battery status updates
```

#
