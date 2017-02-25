use std::net::TcpListener;
use std::env::args;
use std::io;
use std::io::{BufRead, Read, Write};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

fn read_stdin(sender: Sender<Vec<u8>>) {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf.pop();
        sender.send(buf.into_bytes()).unwrap();
    }
}

fn main() {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(format!("listening on {}\n", args().nth(1).unwrap()).as_bytes()).unwrap();
    let (mut stream, address) = TcpListener::bind(args().nth(1).unwrap().as_str()).unwrap().accept().unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
    stdout.write_all(format!("connect from {}\n", address).as_bytes()).unwrap();
    let stdin = {
        let (send, rec) = channel();
        thread::spawn(move || read_stdin(send));
        rec
    };
    loop {
        loop {
            match stdin.try_recv() {
                Ok(val) => {
                    stream.write_all(&val).unwrap();
                    stream.write_all(&[0u8]).unwrap();
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => panic!("input disconnect"),
            }
        }
        let mut read_buf = Vec::new();
        if let Err(e) = stream.read_to_end(&mut read_buf) {
            if e.kind() != io::ErrorKind::WouldBlock {
                panic!(e);
            }
        }
        let out_buf: Vec<u8> = read_buf.iter().map(|b| if *b == b'\0' { b'\n' } else { *b }).collect();
        stdout.write_all(&out_buf).unwrap();
        stdout.flush().unwrap();
    }
}