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

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_types;
extern crate sgx_trts;
extern crate sgx_tcrypto;
extern crate blake2;
extern crate ndarray;

use sgx_types::*;
use sgx_tcrypto::*;
// use sgx_trts::memeq::ConsttimeMemEq;
use std::vec::Vec;
// use std::ptr;
use std::string::String;
use std::string::ToString;
use std::str;
use std::io::{self, Write};
use std::slice;
use std::collections::HashMap;
use blake2::{Blake2b, Digest};
use ndarray::Array;


#[no_mangle]
pub extern "C" fn say_something(lidar: *const Vec<u8>, points_num: usize) -> sgx_status_t {

    println!("result: {:?}", lidar);
    // println!("result: {:?}", lidar_pose);

    // let ecc_handle = SgxEccHandle::new();
    // let _result = ecc_handle.open();
    // let (prv_k, pub_k) = ecc_handle.create_key_pair().unwrap();
    // print!("{:?}", prv_k);
    let s = "Hello, World";
    let t = s.as_bytes();
    let mut hasher = Blake2b::new();

    hasher.input(t);
    // `input` can be called repeatedly and is generic over `AsRef<[u8]>`
    // hasher.input("String data");
    // Note that calling `result()` consumes hasher
    let hash = hasher.result();
    println!("Result: {:x}", hash);
    

    let mut data_to_send = HashMap::new();
    data_to_send.insert(
        "SHA256_Hash".to_string(),
        "This is a test string".to_string(),
    );

 
    sgx_status_t::SGX_SUCCESS
}

