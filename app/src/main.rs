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

extern crate sgx_types;
extern crate sgx_urts;
extern crate blake2;


use blake2::{Blake2b512, Digest};
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::net::TcpStream;
use std::fs;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
    fn say_something(
        eid: sgx_enclave_id_t, 
        retval: *mut sgx_status_t,
        lidar: *const Vec<u8>, 
        points_num: usize
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

fn parse_lidar_pose(file_path: &str) -> Vec<u8> {
    let s = fs::read_to_string(file_path).unwrap();
    let lidar_pose = s.into_bytes();
    return lidar_pose;
}

fn parse_lidar(file_path: &str) -> Vec<u8> {
    let s = fs::read_to_string(file_path).unwrap();
    let lidar = s.into_bytes();
    // let mut hasher = Blake2b512::new();
    // hasher.update(&t);
    // let hash = hasher.finalize();
    // println!("Binary hash: {:#?}", hash);
    return lidar
}

fn main() {
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
    
    let mut lidar_vector: Vec<u8> = parse_lidar("../test/lidar.txt");
    println!("Parsed Lidar");

    let mut lidar_pose: Vec<u8> = parse_lidar_pose("../test/lidar_pose.txt");
    println!("Loaded lidar pose {:?}", lidar_pose);
    
    let lidar = [lidar_vector, lidar_pose].concat();
    println!("Loaded Lidar");
    
    let points_num = lidar.len();
    println!("Loaded lidar image {:?}", points_num);


    let result = unsafe {
        say_something(enclave.geteid(),
                      &mut retval,
                      lidar.as_ptr() as * const Vec<u8>,
                      points_num)
    };
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    println!("[+] say_something success...");


    enclave.destroy();
}
