use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use redis_cli::command::Registry;
use redis_cli::server::process;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let registry = Registry::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");

        handle_connection(stream, &registry);
    }
}

fn handle_connection(mut stream: TcpStream, registry: &Registry) {
    let mut buf: Vec<u8> = Vec::new();
    let mut scratch = [0; 512];
    loop {
        match stream.read(&mut scratch) {
            Ok(0) => {
                break;
            }
            Ok(s) => {
                println!("Read bytes size: {}", s);
                println!("Bytes: {:?}", &scratch[..s]);

                buf.extend_from_slice(&scratch[..s]);
                match process(&mut buf, registry) {
                    Ok(reply) => {
                        stream.write_all(&reply).unwrap_or_else(|e| {
                            println!("Unable to write response: {}", e);
                        });
                    }
                    Err(e) => {
                        println!("Error occured: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("error occured: {}", e);
                break;
            }
        };
    }
}
