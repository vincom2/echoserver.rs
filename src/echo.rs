use std::io::{TcpStream, IoResult};
use std::str;

// should I return IoResult or just unwrap within the function?
// after all, I just want it to die if something bad happens...
pub fn echo(mut in_stream: TcpStream) -> IoResult<()> {
    let mut out_stream = in_stream.clone();
    let msg = in_stream.read_to_end().unwrap();
    print!("{}", str::from_utf8(msg.as_slice()).unwrap());
    out_stream.write(msg.as_slice())
}

