// Implements http://rosettacode.org/wiki/Hello_world/Web_server
#![allow(unused_features)]
#![feature(io)]
#![feature(std_misc)]
#![feature(os)]
#![feature(env)]
#![feature(core)]
#![feature(std_misc)]

use std::old_io::net::tcp::{TcpAcceptor, TcpListener, TcpStream};
use std::old_io::{Acceptor, Listener, IoResult};
use std::thread::Thread;

#[cfg(not(test))] use std::env;

fn handle_client(mut stream: TcpStream) -> IoResult<()> {
    let response =
b"HTTP/1.1 200 OK
Content-Type: text/html;
charset=UTF-8

<doctype !html>
<html>
    <head>
        <title>Bye-bye baby bye-bye</title>
        <style>
            body { background-color: #111 }
            h1 { font-size:4cm; text-align: center; color: black; text-shadow: 0 0 2mm red}
        </style>
    </head>
    <body>
        <h1>Goodbye, world!</h1>
    </body>
</html>";

    try!(stream.write_all(response));
    stream.close_write()
}

pub fn handle_server(ip: &str, port: u16) -> IoResult<TcpAcceptor> {
    let listener = try!(TcpListener::bind((ip, port)));
    let mut acceptor = listener.listen();
    println!("Listening for connections on port {}", port);

    let handle = acceptor.clone();
    Thread::spawn(move || -> () {
        for stream in acceptor.incoming() {
            match stream {
                Ok(s) => {
                    Thread::spawn(move || {
                        match handle_client(s) {
                            Ok(_) => println!("Response sent!"),
                            Err(e) => println!("Failed sending response: {}!", e),
                        }
                    });
                },
                Err(e) => {
                    println!("No longer accepting new requests: {}", e);
                    break
                }
            }
        }
    });

    handle
}

#[cfg(not(test))]
fn main() {
    let mut args = env::args();
    let app_name = args.next().unwrap()
        .into_string().unwrap();
    let host = "127.0.0.1";
    let port = if let Some(os_port) = args.next() {
        let s_port = os_port.into_string().unwrap();
        s_port.parse::<u16>().ok()
            .expect(&*format!("Usage: {:?} <port>", app_name))
    } else {
        80
    };

    handle_server(host, port).unwrap();
}
