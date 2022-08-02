import asyncio
import sys

from bleak import BleakScanner
from bleak import BleakClient
from bleak.backends.device import BLEDevice
from bleak.backends.scanner import AdvertisementData

DEVICE_FILTER = "Tenka"

SERVICE_UUID = "6e400001-b5a3-f393-e0a9-e50e24dcca9e"
UART_TX_UUID = "6e400002-b5a3-f393-e0a9-e50e24dcca9e"
UART_RX_UUID = "6e400003-b5a3-f393-e0a9-e50e24dcca9e"

detected_devices: BLEDevice = []
clients = []


def device_found(device: BLEDevice, ad_data: AdvertisementData):
    if device and device.name and DEVICE_FILTER in device.name:
        detected_devices.append(device)

def client_disconnected(client: BleakClient):
    print("DISCONNECTED: {}".format(client))
    clients.remove(client)

def handle_rx(sender: int, data: bytearray):
    str = data.decode("utf8", errors="ignore")
    print("RX - {}: {}".format(sender, str)) 

def gen_velocity(velocity: int):
    clamped_v = max(-250, min(velocity, 250))
    return "{:+04d}".format(clamped_v)

async def send_check_batt_cmd(client: BleakClient):
    cmd = bytes("b", encoding="utf8")
    print("TX: {}".format(cmd))
    await client.write_gatt_char(UART_TX_UUID, cmd)

async def send_velocity_cmd(client: BleakClient, a: int, b: int, c: int):
    cmd = "{}a{}b{}c".format(gen_velocity(a), gen_velocity(b), gen_velocity(c))
    print("TX: {}".format(cmd))
    await client.write_gatt_char(UART_TX_UUID, bytes(cmd, encoding="utf8"))

async def connect(device: BLEDevice):
    client = BleakClient(device.address, disconnected_callback=client_disconnected)
  
    await client.connect()
    await client.start_notify(UART_RX_UUID, handle_rx)
    await send_check_batt_cmd(client)
    
    print("Connected to {}".format(device.name))
    clients.append(client)


async def main():

    print("Scanning for Tenka CircuitCube BLE devices...")    
    await BleakScanner.find_device_by_filter(device_found, 1)
    print("Scan Complete.")

    print("Found {} Tenka Circuit Cubes.".format(len(detected_devices)))
    for device in detected_devices:
        print(device)
        
    print("Connecting to Circuit Cubes...")
    for device in detected_devices:
        await connect(device)

#    asyncio.gather(*(connect(device) for device in detected_devices))

    print("Running Main Loop...")
    while len(clients) > 0:
        for client in clients:
            await send_velocity_cmd(client, 0, 0, 0)
        await asyncio.sleep(1)

        for client in clients:
            await send_velocity_cmd(client, 50, 150, 250)
        await asyncio.sleep(1)

        for client in clients:
            await send_velocity_cmd(client, 0, 0, 0)
        await asyncio.sleep(1)

        for client in clients:
            await send_velocity_cmd(client, -50, -250, -250)
        await asyncio.sleep(1)

asyncio.run(main())
