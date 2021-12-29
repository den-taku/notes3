use log::*;
use simplelog::*;
use std::fs::File;

struct MyWriter {
    file: File,
}

impl std::io::Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        if buf.len() == 8
            && 48 <= buf[0]
            && buf[0] <= 57
            && 48 <= buf[1]
            && buf[1] <= 57
            && buf[2] == 58
            && 48 <= buf[3]
            && buf[3] <= 57
            && 48 <= buf[4]
            && buf[4] <= 57
            && buf[5] == 58
            && 48 <= buf[6]
            && buf[6] <= 57
            && 48 <= buf[7]
            && buf[7] <= 57
        {
            self.file
                .write(
                    &"Hello! This is Dentaku. : "
                        .chars()
                        .map(|c| c as u8)
                        .collect::<Vec<u8>>(),
                )
                .unwrap();
        }
        // print!("{} ", buf.len());
        // let mut buffer = "This is new type of log: "
        //     .chars()
        //     .map(|c| c as u8)
        //     .collect::<Vec<u8>>();
        // buffer.append(&mut buf.iter().map(|c| *c).collect::<Vec<u8>>());
        // println!(
        //     "{}: {}",
        //     buffer.iter().map(|u| *u as char).collect::<String>(),
        //     buffer.len()
        // );
        // self.file.write(&buffer)
        println!("{:?}", buf);
        self.file.write(&buf)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        self.file.flush()
    }
}

impl MyWriter {
    fn new(filename: &'static str) -> Self {
        Self {
            file: File::create(filename).unwrap(),
        }
    }
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("my_rust_binary.log").unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            MyWriter::new("my_rust_binary2.log"),
        ),
    ])
    .unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
    error!("End!");
}
