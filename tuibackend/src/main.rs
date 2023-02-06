use std::time::Duration;


fn main() 
    {
    println!("Hello, world!");

    let mut serial_read = SerialRead::new();

    loop
        {
        serial_read.read_serial_broken();
        }
    }

struct SerialRead
    {
    port: Box<dyn serialport::SerialPort>,
    serial_str: String,
    processed: Vec<String>
    }

impl SerialRead
    {
    fn new() -> SerialRead
        {
        let port = serialport::new("/dev/ttyACM0", 9600).timeout(Duration::from_millis(10)).open()
            .expect("unable to open serialport");

        let serial_str = String::new();
        let processed: Vec<String> = Vec::new();

        SerialRead
            {
            port,
            serial_str,
            processed,
            }
        }

    #[allow(dead_code)]
    fn read_serial_unbroken(&mut self)
        {
        let mut serial_buf: Vec<u8> = vec![0; 100];
        self.port.read(serial_buf.as_mut_slice()).expect("unable to read serialport");
        self.serial_str = String::from_utf8(serial_buf.clone()).expect("unable to convert serial_buf to string");
        println!("{:?}", self.serial_str);
        println!("{:?}", self.port.bytes_to_read().expect("unable to read bytes_to_read"));
        }

    fn read_serial_broken(&mut self)
        {
        let bytes_to_read = self.port.bytes_to_read().expect("unable to read bytes_to_read");
        if bytes_to_read > 0
            {
            let mut buf = vec![0; bytes_to_read.try_into().unwrap_or(0)];
            self.port.read_exact(&mut buf).unwrap();
            self.serial_str = String::from_utf8(buf.clone()).expect("unable to convert serial_buf to string");
            self.processed = self.serial_str.split_whitespace().map(|s| s.to_string()).collect();
            }
        // println!("{:?}", self.processed);
        }
    }
