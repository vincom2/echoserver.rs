use std::io::{TcpListener, TcpStream};
use std::io::net::tcp::TcpAcceptor;
use std::io::{Acceptor, Listener};
use std::io::IoResult;

fn main() {
    let mut acceptor = TcpListener::bind("127.0.0.1", 80).listen().unwrap();
    let stream : Option<IoResult<TcpStream>> = acceptor.incoming().next();
    //for stream in acceptor.incoming() {
    //}
}

