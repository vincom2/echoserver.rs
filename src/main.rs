extern crate getopts;

use bind::try_bind;
use echo::echo;
use getopts::{optopt, optflag, getopts, OptGroup};
use std::io::{Acceptor, Listener};
use std::os;

mod bind;
mod echo;

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

    let port : u16 = match matches.opt_str("p") {
        None => 60002,
        Some(p) => from_str(p.as_slice()).unwrap()
    };

    let mut acceptor = try_bind("127.0.0.1", port).unwrap().listen();
    for stream in acceptor.incoming() {
        let in_stream = stream.unwrap();
        spawn(proc() {
            echo(in_stream).unwrap();
        })
    }
}

