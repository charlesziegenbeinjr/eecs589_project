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




#[no_mangle]
pub extern "C" fn say_something(lidar: *const u8, points_num: usize) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(lidar, points_num) };
    println!("Resulting Str_Slice Length: {:?}", str_slice.len());
    // let mut lidar_deref = *lidar
    // let lidar_2: &[u8] = lidar.as_ref();
    // let asref = &str_slice.as_ref()

    // let ecc_handle = SgxEccHandle::new();
    // let _result = ecc_handle.open();
    // let (prv_k, pub_k) = ecc_handle.create_key_pair().unwrap();
    // print!("{:?}", prv_k);

    // let s = "Hello, World";
    // let t = s.as_bytes();
    let mut hasher = Blake2b::new();
    hasher.input(str_slice);
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

