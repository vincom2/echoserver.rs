extern crate getopts;
use getopts::{optopt, optflag, getopts, OptGroup};
use std::str;
use std::io::{TcpListener, Acceptor, Listener};
use std::os;

fn print_usage(program: &str, opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    for &o in opts.iter() {
        //should print to string first in case option has no name?
        println!("-{} --{}\t{}", o.short_name, o.long_name, o.desc);
    }
}

fn main() {
    let args = os::args();
    let program = args[0].clone();
    let opts = [ optopt("p", "port", "port to listen on (default 60002)", "NUMBER"),
                 optflag("h", "help", "print this") ];
    let matches = getopts(args.tail(), opts).unwrap();
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let port : u16= match matches.opt_str("p") {
        None => 60002,
        Some(p) => from_str(p.as_slice()).unwrap()
    };

    let mut acceptor = TcpListener::bind("127.0.0.1", port).listen();
    for stream in acceptor.incoming() {
        let mut in_stream = stream.unwrap();
        let mut out_stream = in_stream.clone();
        let msg = in_stream.read_to_end().unwrap();
        print!("{}", str::from_utf8(msg.as_slice()).unwrap());
        out_stream.write(msg.as_slice()).unwrap();
    }
}

