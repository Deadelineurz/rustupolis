use std::{env, fs, io};
use std::fs::OpenOptions;
use std::io::{stdout, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use termion::{clear, cursor};
use rustopolis::logging::get_logger_socket_path;

fn get_path() -> PathBuf {
    PathBuf::from(format!("{}/rustopolis/logger.socket", env::var("XDG_RUNTIME_DIR").unwrap()).to_string())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    let p = get_logger_socket_path();
    let mut stdout = stdout();

    fs::create_dir_all(&p.parent().unwrap()).expect("Could not create dir");
    let _ = fs::remove_file(&p);
    let listener = UnixListener::bind(&p).expect("Could not create socket");

    let mut resp = String::new();

    println!("{}{}Waiting for connections", clear::All, cursor::Goto(1, 1));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.read_to_string(&mut resp).unwrap();
                print!("{}", resp);
                stdout.flush().unwrap()
            }
            Err(err) => {
                break
            }
        }
    }
}