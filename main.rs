use std::str;
use std::io::{TcpListener, Acceptor, Listener};

fn main() {
    let mut acceptor = TcpListener::bind("127.0.0.1", 60002).listen();
    for stream in acceptor.incoming() {
        let mut in_stream = stream.unwrap();
        let mut out_stream = in_stream.clone();
        let msg = in_stream.read_to_end().unwrap();
        print!("{}", str::from_utf8(msg.as_slice()).unwrap());
        out_stream.write(msg.as_slice()).unwrap();
    }
}

