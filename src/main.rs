use std::sync::mpsc::{Receiver, channel};
use std::thread;
use std::io::{Read};
use std::net::TcpListener;

fn get_lines_channel(mut f: impl Read + Send + 'static) -> Receiver<String> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut buffer = [0; 8]; // Create a buffer of 8 bytes
        let mut one_line = String::new();    
        loop {
            match f.read(&mut buffer) {
             Ok(bytes_read) => {
                if bytes_read == 0 {
                if !one_line.is_empty() {
                    tx.send(one_line).unwrap();
                }
                // End of file
                break;
                }
                
                if let Ok(text) = std::str::from_utf8(&buffer[..bytes_read]) {

                    let index = text.find("\n");

                    match index {
                        Some(i) => {
                            if i != 0 {
                                one_line.push_str(&text[0..i]);
                            }
                            tx.send(one_line).unwrap();
                            one_line = text[i+1..].to_string();
                        },
                        None => one_line.push_str(text)
                    }
                } else {
                    println!("(Contains non-UTF8 data)");
                }
             },
             Err(e) => tx.send(e.to_string()).unwrap(),
            }
            
        }
    });
    
    rx

}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ln = TcpListener::bind("127.0.0.1:42069")?;

    for stream in ln.incoming() {
        match stream {
            Ok(s) => {
                let receiver = get_lines_channel(s);

                for line in receiver {
                    println!("{}",line);
                }
            },
            Err(e) => println!("{}", e),
        }
    }
   
    
        
    Ok(())
}
