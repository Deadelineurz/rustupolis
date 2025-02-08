use ansi_term::Colour;
use chrono::Local;
use log::{info, Level, Metadata, Record};
use std::io::Write;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Mutex;
use std::{env, process};
use termion::color;
use termion::color::Fg;

#[cfg(not(target_os = "macos"))]
pub fn get_logger_socket_path() -> PathBuf {
    PathBuf::from(format!("{}/rustupolis/logger.socket", env::var("XDG_RUNTIME_DIR").unwrap_or(
        format!("/run/user/{}", users::get_current_gid())
    )).to_string())
}

#[cfg(target_os = "macos")]
pub fn get_logger_socket_path() -> PathBuf {
    PathBuf::from(format!("/tmp/rustupolis/logger.socket"))
}

pub struct RemoteLoggerClient {
    level: Level,
    stream: Mutex<Option<UnixStream>>
}

impl RemoteLoggerClient {
    pub fn new() -> Self {
        let mut stream = UnixStream::connect(get_logger_socket_path()).ok();

        match &mut stream {
            Some(s) => {
                write!(s, "{}Rustupolis connected | PID: {}\n", Fg(color::Reset), process::id())
            }
            _ => {Ok(())}
        }.expect("TODO: panic message");

        RemoteLoggerClient{
            level: Level::Trace,
            stream: Mutex::new(stream)
        }
    }
}

impl Drop for RemoteLoggerClient {
    fn drop(&mut self) {
        match self.stream.lock() {
            Ok(mut guard) => {
                match &mut *guard {
                    Some(stream) => {
                        stream.shutdown(Shutdown::Both).unwrap();
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }
}

fn get_message(record: &Record) -> String {
    let mut s = String::new();

    let color = match record.level() {
        Level::Error => {
            Colour::Red
        }
        Level::Warn => {
            Colour::Yellow
        }
        Level::Info => {
            Colour::Green
        }
        Level::Debug | Level::Trace => {
            Colour::Cyan
        }
    };

    s += &color.prefix().to_string();

    s += &format!("{}[{}] ", color.prefix(), Local::now().format("%F %T%.3f"));

    match (record.file(), record.line()) {
        (Some(f), Some(l)) => {
            s += &format!("({}:{:?}) ", f, l)
        },
        (Some(f), None) => {
            s += &format!("({}) ", f)
        }
        (None, _) => {

        }
    }

    s += &format!("- {}{}{}{} - {}", color.reverse().prefix(), record.level(), color.reverse().suffix(), color.prefix(), record.args());

    s.push('\n');

    s
}

impl log::Log for RemoteLoggerClient {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        match self.stream.lock() {
            Ok(mut guard) => {
                match &mut *guard {
                    Some(stream) => {
                        let _ = write!(stream, "{}", get_message(record));
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }

    fn flush(&self) {

    }
}