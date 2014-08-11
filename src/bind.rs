use std::io::{TcpListener, stdio};

// there's probably a better way to do this...
pub fn try_bind(host: &str, port: u16) -> Result<TcpListener, &str> {
    println!("trying to bind to {}:{}", host, port);
    match TcpListener::bind("127.0.0.1", port) {
        Err(e) => {
            println!("Error: {}", e);
            let mut input = stdio::stdin();
            print!("Would you like to try another port? (y/n): ");
            let resp = input.read_char().unwrap();
            if resp == 'Y' || resp == 'y' {
                // my god do you know how long it took me to fix getting input :'(
                let _ = input.read_line(); //consume CR
                print!("Enter new port: ");
                let tmp = input.read_line().unwrap(); //grr, fix this already mozilla people
                let tmp2 = tmp.as_slice().trim_right_chars('\n'); 
                let p : u16 = from_str(tmp2).unwrap();
                try_bind(host, p)
            }
            else { Err("Unable to bind to port") }
        }
        Ok(b) => {print!("listening!\n\n"); Ok(b)}
    }
}

