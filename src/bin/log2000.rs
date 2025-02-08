use rustupolis::logging::get_logger_socket_path;
use std::io::{stdout, Read, Write};
use std::os::unix::net::UnixListener;
use std::fs;
use std::time::Duration;
use termion::{clear, color, cursor};

fn main() {
    let p = get_logger_socket_path();
    let mut stdout = stdout();

    fs::create_dir_all(&p.parent().unwrap()).expect("Could not create dir");
    let _ = fs::remove_file(&p);
    let listener = UnixListener::bind(&p).expect("Could not create socket");

    println!("{}{}Waiting for connections", clear::All, cursor::Goto(1, 1));

    let mut buf = [0u8; 1024];

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.set_read_timeout(Some(Duration::from_secs(30))).expect("TODO: panic message");
                while let Ok(read) = stream.read(&mut buf) {
                    if read == 0 {
                        break
                    }
                    let _ = stdout.write(&buf[..read]);
                    let _ = stdout.flush();
                }
            }
            Err(_) => {
                break
            }
        }

        println!("{}Connection closed", color::Reset.fg_str());
    }
}