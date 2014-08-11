use std::str;
use std::io::{TcpListener, Acceptor, Listener};

fn main() {
    let mut acceptor = TcpListener::bind("127.0.0.1", 60002).listen();
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => fail!("died with exception: {}", e),
            Ok(mut in_stream) => {
                let mut out_stream = in_stream.clone();
                let msg = in_stream.read_to_end().unwrap();
                print!("{}", str::from_utf8(msg.as_slice()).unwrap());
                out_stream.write(msg.as_slice());
            }
        }
    }
}

