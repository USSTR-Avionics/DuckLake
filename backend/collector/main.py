import datetime
import serial

portVar = '/dev/ttyACM0'

serialInst = serial.Serial()
serialInst.baudrate = 9600
serialInst.port = portVar
serialInst.open()

def main():
    while True:
        if serialInst.in_waiting:
            packet = serialInst.readline()
            line = packet.decode('utf').strip('\n')
            curr_datetime = datetime.datetime.now().strftime('%Y-%m-%d %H:%M:%S')
            print(curr_datetime, line)
            # write to a sql server





if __name__ == '__main__':
    try:
        main()
    except Exception as e:
        print(e)
