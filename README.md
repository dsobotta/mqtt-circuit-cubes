# mqtt-circuit-cubes
Tools to enable MQTT support for Tenka Circuit Cubes


# Installation
```
  sudo apt update
  sudo apt upgrade
  
  sudo apt install python pip mosquitto mosquitto-clients
  pip install bleak
  pip install paho-mqtt
  
  git clone https://github.com/dsobotta/mqtt-circuit-cubes.git
  cd mqtt-circuit-cubes/python
  pip install .
  
```

# Usage
```
  chmod +x examples/*
  examples/mqtt-cc-bridge
  
  #in a separate terminal session...
  mosquitto_pub -h localhost -t "<CC_DEVICE_ID>/cmd/power/a" -m "250" #full power
  mosquitto_pub -h localhost -t "<CC_DEVICE_ID>/cmd/power/b" -m "-250" #full reverse power
  mosquitto_pub -h localhost -t "<CC_DEVICE_ID>/cmd/power/c" -m "0" #power off
  mosquitto_sub -h localhost -t "<CC_DEVICE_ID>/status/battery" #subscribe to battery status updates
```

#
