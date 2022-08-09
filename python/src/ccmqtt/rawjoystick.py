import os
import struct
import array
from fcntl import ioctl
from typing import Callable

#lists all discovered joysticks
def list_joysticks():
    print("discovered joysticks:")

    for fn in os.listdir('/dev/input/'):
        if fn.startswith('js'):
            print('  /dev/input/%s' % (fn))
    return
    
class RawJoystick:
    
    path = ""
    device_id = ""
    name = ""
    device = None

    num_axes = 0
    axis_states = {}

    num_buttons = 0
    button_states = {}

    button_callbacks: list[Callable[["RawJoystick", int, int], None]] = list()
    axis_callbacks: list[Callable[["RawJoystick", int, int], None]] = list()

    def __init__(self, filename):
        
        #open device
        self.path = filename
        self.device_id = os.path.basename(filename)

        print("opening joystick %s..." % filename)
        self.device = open(filename, 'rb')

        #device name
        buf = array.array('B', [0] * 64)
        ioctl(self.device, 0x80006a13 + (0x10000 * len(buf)), buf) # JSIOCGNAME
        self.name = buf.tobytes().rstrip(b'\x00').decode("utf8")
        print("device name: %s" % self.name)

        #map axes
        buf = array.array('B', [0])
        ioctl(self.device, 0x80016a11, buf) # JSIOCGAXES
        self.num_axes = buf[0]

        buf = array.array('B', [0] * 0x40)
        ioctl(self.device, 0x80406a32, buf) # JSIOCGAXMAP
        for axis in buf[:self.num_axes]:
            self.axis_states[axis] = 0.0

        #map buttons
        buf = array.array('B', [0])
        ioctl(self.device, 0x80016a12, buf) # JSIOCGBUTTONS
        self.num_buttons = buf[0]
        
        buf = array.array('H', [0] * 200)
        ioctl(self.device, 0x80406a34, buf) # JSIOCGBTNMAP
        for button in buf[:self.num_buttons]:
            self.button_states[button] = 0

        print("%d axes found" % self.num_axes)
        print("%d buttons found" % self.num_buttons)

    
    def add_axis_listener(self, function: Callable[["RawJoystick", int, int], None]):
        self.axis_callbacks.append(function)

    def add_button_listener(self, function: Callable[["RawJoystick", int, int], None]):
        self.button_callbacks.append(function)

    def remove_axis_listener(self, function: Callable[["RawJoystick", int, int], None]):
        self.axis_callbacks.remove(function)

    def remove_button_listener(self, function: Callable[["RawJoystick", int, int], None]):
        self.button_callbacks.remove(function)

    #continualy reads updates from joystick device. publishes events to button and axis listeners
    def update_blocking(self):
        
        buf = self.device.read(8)
        while(buf):
            time, value, type, number = struct.unpack('IhBB', buf)
            
            #handle buttons
            if type & 0x01:
                self.button_states[number] = value
                for callback in self.button_callbacks:
                    callback(self, number, value)
    
            #handle axes
            elif type & 0x02:
                fvalue = value / 32767.0
                self.axis_states[number] = fvalue
                for callback in self.axis_callbacks:
                    callback(self, number, int(fvalue))

            buf = self.device.read(8)

