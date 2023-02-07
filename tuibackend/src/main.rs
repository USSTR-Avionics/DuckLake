use std::{time::Duration, thread, io, default};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::{Span, Spans},
    Terminal, Frame};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, self},
    };

struct App {
    titles: Vec<String>,
    index: usize,
    }

impl App {
    fn new() -> App {
        App {
            titles: vec!["Dashboard".to_string(), "Launch Control".to_string()],
            index: 0,
            }
        }
    fn next_tab(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    fn previous_tab(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}


fn main() -> Result<(), io::Error>
    {
    println!("Hello, world!");

    // setup terminal
    enable_raw_mode().expect("unable to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("unable to enter alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("unable to create terminal");

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode().expect("unable to disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
        ).expect("unable to leave alternate screen");

    terminal.show_cursor().expect("unable to show cursor");

    Ok(())
    }

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                ].as_ref(),
            )
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, size);

    let titles = app.titles.iter().map(|t| {
        let (first, rest) = t.split_at(1);
        Spans::from(vec![
            Span::styled(first, Style::default().fg(Color::Red)),
            Span::styled(rest, Style::default().fg(Color::Black)),
            ])
    }).collect();
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

fn loop_read_serial()
    {
    let mut serial_read = SerialRead::new();
    let mut last_read_first = String::new();

    loop {
        serial_read.read_serial_broken();
        if serial_read.processed[0] != last_read_first
            {
            println!("{:?}", serial_read.processed);
            }
        last_read_first = serial_read.processed[0].clone();
        }

    }
