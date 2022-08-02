import asyncio
import sys

from bleak import BleakScanner

def device_found(device, ad_data):
    if device and device.name and "Tenka" in device.name:
        print(device)

async def main(filter):
    print("Scanning for Tenka CircuitCube BLE devices...")
    await BleakScanner.find_device_by_filter(device_found)
    print("Scan Complete.")

asyncio.run(main("Tenka"))
