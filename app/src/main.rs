// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..
#![feature(tcp_linger)]

extern crate sgx_types;
extern crate sgx_urts;
extern crate hex;

use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::fs;
use hex::encode;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::thread;
use std::io::{Read,Write,Error};
use std::time::Duration;
use std::time::Instant;
    
// use serde_json;


static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
    fn say_something(
        eid: sgx_enclave_id_t, 
        retval: *mut sgx_status_t,
        lidar: *const u8, 
        points_num: usize,
        hash: *mut [u8;64]
    ) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}


fn main() {
    // Start the Timing for the System Here...
    let now = Instant::now();

    // Initialize the Enclave
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;
    
    let lidar_string: String = fs::read_to_string("../test/2005_000069_anomaly.txt").unwrap();
    let hello: String = lidar_string.replace("\n", "ZZ");
    println!("Parsed Lidar");
    println!("Parsed Lidar Length {:?}", hello.len());

    let lidar_pose: String = fs::read_to_string("../test/2005_000069_lidar_pose.txt").unwrap();
    println!("Loaded lidar pose {:?}", lidar_pose);
    println!("Lidar Pose Length {:?}", lidar_pose.len());
    
    
    let lidar = format!("{}{}", hello, lidar_pose);
    
    let points_num = lidar.len();
    println!("Loaded lidar image {:?}", points_num);

    let hash_app = [0; 64];
    println!("Initial Hash from App:{:?}", hash_app);


    let result = unsafe {
        say_something(enclave.geteid(),
                    &mut retval,
                    lidar.as_ptr() as * const u8,
                    points_num,
                    hash_app.as_ptr() as * mut [u8;64])
    };
    
    let to_hex = encode(hash_app);
    println!("Returned Hash from Enclave in Hex: {:?}", to_hex);

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    // **********************************************************

    // At this point, we have what we need from encode(),
    // and we can convert into a &[u8].
    // To do this, use as_bytes on the to_hex variable

    let lidar_string_asBytes = hello.as_bytes();
    println!("Converted Lidar PCD To &[u8]");
    let lidar_pose_asBytes = lidar_pose.as_bytes();
    println!("Converted Lidar Pose To &[u8]");
    let hash_to_send = to_hex.as_bytes();
    println!("Converted Hash To &[u8]");

    println!("[+] say_something success...");
    
    let connection = send_data(lidar_string_asBytes, lidar_pose_asBytes ,hash_to_send);
    
    let elapsed = now.elapsed();
    println!("Execution Time: {:.2?}", elapsed);

    let listener = TcpListener::bind("172.17.0.2:8080").expect("Error");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("Failed: {}", e)}
            Ok(stream) => {
                println!("Receiver Processed Data, Shutting Down");
                let final_time = now.elapsed();
                println!("Total Round Trip Execution Time: {:.2?}", final_time);
                break;
            }
        }
    }
    enclave.destroy();
}

fn send_data(lidar: &[u8], lidar_pose: &[u8], hash: &[u8]) -> Result<(),Error> {
    let addr = SocketAddr::from(([172, 17, 0, 3], 8080));
    
    let mut stream = TcpStream::connect_timeout(&addr,Duration::from_secs(10))?;
    let random = "|";
    let rand = random.as_bytes();

    println!("Outgoing Connection Started");
    stream.write(lidar)?;
    stream.flush()?;
    stream.write(rand)?;
    stream.flush()?;
    stream.write(lidar_pose)?;
    stream.flush()?;
    stream.write(rand)?;
    stream.flush()?;
    stream.write(hash)?;
    stream.flush()?;
    Ok(())
}
    
    // let mut stream = TcpStream::connect("https://cbzjr.com")?;
    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;
    // Ok(())