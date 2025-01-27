#!/usr/bin/python3

import sys
import usb
import datetime
import struct

devices = []
for device in usb.core.find(find_all=True, idVendor=0x0fe7, idProduct=0x4001):
    # Only one configuration, interface, and endpoint for the USB-ITN cable
    device.reset()
    device.set_configuration(1)
    epin = device.get_active_configuration().interfaces()[0].endpoints()[0]

    # Idk this is magic?
    bmRequestType=0x40 # Vendor Host-to-Device
    device.ctrl_transfer(bmRequestType, 0x1, 0xa5a5, 0)

    bmRequestType=0xC0 # Vendor Device-to-Host
    res1 = device.ctrl_transfer(bmRequestType, 0x2, 0, 0, 1)
    print("Device Vendor resp: {}".format(res1))

    # Get some version/serial thing? Is this per cable? I think so. It's
    # not the actual end-device ID but w/e it's enough to uniquely
    # identify the connection
    device.ctrl_transfer(0x40, 0x3, 0, 0, "V\r")

    while True:
        raw = epin.read(64)
        if len(raw) == 0:
            continue

        serial = raw.tobytes().decode()[1:-1]
        print(raw)
        print("Got serial " + serial)
        break

    log = open(f"indicator_readings_{serial}.log", "ab")
    devices.append((serial, device, epin, log))

while True:
    DATA_MSG='0'
    DEVICE_INFO_MSG='1'
    STATUS_MSG='9'

    # Request the reading from all devices
    for (_, device, _, _) in devices:
        device.ctrl_transfer(0x40, 0x3, 0, 0, "1\r")

    # Read the data from all devices
    readings = []
    for (_, _, epin, _) in devices:
        raw_data = epin.read(64)
        cur_time = datetime.datetime.now(datetime.timezone.utc)
        readings.append((raw_data, cur_time))

    for ((raw_data, cur_time), (serial, _, _, fd)) in zip(readings, devices):
        # Convert the reading to a string
        reading = raw_data.tobytes().decode()
        msg_type = reading[:1]
        payload = reading[1:]

        if msg_type == DATA_MSG:
            # Never seen anything different
            assert reading[1:3] == '1A'
            assert reading[-1:] == '\r'
            mm = float(reading[3:-1])

            print(f"[{cur_time}] [{serial}]: {mm:12.4f}")

            # Record the timestamped data
            fd.write(struct.pack("<dd", cur_time.timestamp(), mm))
            fd.flush()
        elif msg_type == DEVICE_INFO_MSG:
            print(f"Device info : {payload}")
        elif msg_type == STATUS_MSG:
            if payload == '18\r':
                # Ignore "not ready" or whatever this message is
                # We get them until data is available again.
                continue

            print(f"Device status : {payload}")
        else:
            print(f"Unknown device response : {reading[:1]}")

