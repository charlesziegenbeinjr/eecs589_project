//extern crate serde;
//extern crate serde_json;



use std::net::{TcpStream, TcpListener};
use std::thread;
use std::io::{Read,Write,Error};
//use serde::{Serialize, Deserialize};
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Error");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("Failed: {}", e)}
            Ok(stream) => {
                // thread::spawn(move || {send_data(stream).unwrap_or_else(|error| eprintln!("{:?}", error))});
                send_data(stream);
            }
        }
    }

}


fn send_data(mut stream: TcpStream) {
    // println!("Incoming Connection From: {}", stream.peer_addr()?);
    let mut edited_line = String::new();
    for data in BufReader::new(&mut stream).lines() {
        let header = data.unwrap();
        edited_line.push_str(&header);
        edited_line.push_str("/n");
        // println!("{}", header);
        // if header == "" {
        //     println!("HERE");
        //     break;
        // }
    }
    println!("Edited: {:?}", edited_line)
    // let mut buf = [0;2048];
    // loop {
    //     let bytes_read = stream.read(&mut buf)?;
    //     //let mut reader = BufReader::new(bytes_read);
    //     println!("Bytes that were read {:?}", bytes_read);
    //     println!("Buffer Contents: {:?}", buf);
    //     if bytes_read == 0 {
    //         return Ok(())
    //     }
    //     stream.write(&buf[..bytes_read])?;
    // }

    // let mut stream = TcpStream::connect("https://cbzjr.com")?;
    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;
    // Ok(())
}
