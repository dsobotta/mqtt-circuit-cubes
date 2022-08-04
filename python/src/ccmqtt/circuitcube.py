
DEVICE_FILTER = "Tenka"

SERVICE_UUID = "6e400001-b5a3-f393-e0a9-e50e24dcca9e"
UART_TX_UUID = "6e400002-b5a3-f393-e0a9-e50e24dcca9e"
UART_RX_UUID = "6e400003-b5a3-f393-e0a9-e50e24dcca9e"

class CircuitCubeTranslator:

    __devicename: str = ""
    __mqtt_topic_battery: bytes = ""
    __mqtt_topic_power: bytes = ""

    def __init__(self, devicename):
        self.__devicename = devicename
        self.__mqtt_topic_battery = bytes("{}/status/battery".format(devicename), encoding="utf8")
        self.__mqtt_topic_power = bytes("{}/cmd/power".format(devicename), encoding="utf8")

    #human-readable device name. Typically matches the BLE device name.
    @property
    def devicename(self) -> str:
        return self.__devicename

    #mqtt topic for getting battery charge
    @property
    def mqtt_topic_battery(self) -> bytes:
        return self.__mqtt_topic_battery

    #mqtt topic for setting power levels
    @property
    def mqtt_topic_power(self) -> bytes:
        return self.__mqtt_topic_power


    def __gen_power_substr(self, power: int) -> str:
        clamped_power = max(-250, min(power, 250) )
        return "{:+04d}".format(clamped_power)


    #decodes data received from BLE/UART RX characteristic
    def uart_decode_rx(self, data: bytes) -> str:
        return data.decode("utf8", errors="ignore")

    #formats a byte array for sending a 'get battery charge' command over BLE/UART
    #response 
    def uart_cmd_battery(self) -> bytes:
        cmd = "{}".format("b")
        return bytes(cmd, encoding="utf8")

    #formats a byte array for sending a 'set power' command over BLE/UART
    #power clamped to range [-250,250] for each output
    def uart_cmd_power(self, a: int, b: int, c: int) -> bytes:
        pwr_a = self.__gen_power_substr(a)
        pwr_b = self.__gen_power_substr(b)
        pwr_c = self.__gen_power_substr(c)
        cmd = "{}a{}b{}c".format(pwr_a, pwr_b, pwr_c)
        return bytes(cmd, encoding="utf8")

    


