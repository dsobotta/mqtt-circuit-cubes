import asyncio
import paho.mqtt.client as mqtt
from bleak import BleakScanner
from bleak import BleakClient
from bleak.backends.device import BLEDevice
from bleak.backends.scanner import AdvertisementData

DEVICE_FILTER = "Tenka"
SERVICE_UUID = "6e400001-b5a3-f393-e0a9-e50e24dcca9e"
UART_TX_UUID = "6e400002-b5a3-f393-e0a9-e50e24dcca9e"
UART_RX_UUID = "6e400003-b5a3-f393-e0a9-e50e24dcca9e"

class BLEClient:

    __device: BLEDevice = None
    __ble_client: BleakClient = None
    __last_batt_read: int  = 0
    __battery_level: float = 0

    def __init__(self, device: BLEDevice):
        self.__device = device
        self.__ble_client = BleakClient(device.address, disconnected_callback=self.__on_client_disconnected)

    async def connect(self):
        await self.__ble_client.connect()
        await self.__ble_client.start_notify(UART_RX_UUID, self.__handle_rx)

    async def disconnect(self):
        await self.__ble_client.disconnect()

    @property
    def name(self) -> str:
        return self.__device.name

    def __on_client_disconnected(client: BleakClient):
        print("client disconnected: {}".format(client))

    def __handle_rx(self, sender: int, data: bytearray) -> None:
        msg = data.decode("utf8", errors="ignore")
        #print("RX:{}".format(msg))
        if  len(msg) > 1:
            self.__battery_level = float(msg)
            self.__last_batt_read += 1

    async def __handle_tx(self, cmd: str) -> None:
        #print("{} TX: {}".format(self.name, cmd))
        encoded = bytes(cmd, encoding="utf8")
        await self.__ble_client.write_gatt_char(UART_TX_UUID, encoded)

    async def __wait_until_batt_updated(self, curr_read: int):
        while curr_read == self.__last_batt_read:
            await asyncio.sleep(0.01)

    def __gen_power_cmd(self, power: int, output: str) -> str:
        clamped_power: int = max(-255, min(power, 255))
        return "{:+04d}{}".format(clamped_power, output)

    #sets power for the given output {a,b,c} to range [-255,255]
    async def set_power(self, power: int, output: str) -> None:
        cmd = self.__gen_power_cmd(power, output)
        await self.__handle_tx(cmd)

    #get battery level in volts
    async def get_battery(self) -> float:
        await asyncio.gather(
            self.__wait_until_batt_updated(self.__last_batt_read),
            self.__handle_tx("b")
        )
        return self.__battery_level


class MQTTBridge:
    
    __ble_client: BLEClient = None
    __mqtt_client: mqtt.Client = None
    __event_loop = None
    __base_topic = ""

    def __init__(self, device:BLEDevice):
        self.__ble_client = BLEClient(device)
        self.__mqtt_client = mqtt.Client()
        self.__base_topic = "{}/cmd/power".format(device.name)
        self.__mqtt_client.message_callback_add("{}/a".format(self.__base_topic), self.__on_msg_cmd_a)
        self.__mqtt_client.message_callback_add("{}/b".format(self.__base_topic), self.__on_msg_cmd_b)
        self.__mqtt_client.message_callback_add("{}/c".format(self.__base_topic), self.__on_msg_cmd_c)

    async def connect(self, mqtt_ip: str) -> None:
        await self.__ble_client.connect()
        self.__mqtt_client.connect(mqtt_ip, 1883, 60)
        self.__mqtt_client.subscribe("{}/#".format(self.__base_topic))
        self.__mqtt_client.loop_start()
        self.__event_loop = asyncio.get_event_loop()

    async def disconnect(self) -> None:
        self.__mqtt_client.loop_stop()
        self.__mqtt_client.disconnect()
        self.__ble_client.disconnect()

    def __send_pwr_msg(self, msg, output) -> None:
        power = int(msg.payload)
        print("Relaying {} - {}".format(str(msg.topic), power))
        self.__event_loop.create_task(self.__ble_client.set_power(power, output))

    def __on_msg_cmd_a(self, mosq, obj, msg):
        self.__send_pwr_msg(msg, "a")
    
    def __on_msg_cmd_b(self, mosq, obj, msg):
        self.__send_pwr_msg(msg, "b")
    
    def __on_msg_cmd_c(self, mosq, obj, msg):
        self.__send_pwr_msg(msg, "c")

    
