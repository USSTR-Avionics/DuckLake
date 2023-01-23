import datetime
import serial
import time

baudrate = 9600
port = '/dev/ttyACM0'
timeout = 1

while(True):
    try:
        with serial.Serial(port, baudrate, timeout=timeout) as ser:
            line = ser.readline().strip()
            curr_datetime = datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            print(curr_datetime, int.from_bytes(line, byteorder='little'))
            time.sleep(0.250)
    except Exception as e:
        continue
