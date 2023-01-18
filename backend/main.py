import datetime
import serial

baudrate = 9600
port = '/dev/ttyUSB0'
timeout = 1

with serial.Serial(port, baudrate, timeout=timeout) as ser:
    line = ser.readline()
    curr_datetime = datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')
    print(curr_datetime, line)
