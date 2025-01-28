use rustupolis::logging::get_logger_socket_path;
use std::io::{stdout, Read, Write};
use std::os::unix::net::UnixListener;
use std::fs;
use termion::{clear, cursor};

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
            Err(_) => {
                break
            }
        }
    }
}