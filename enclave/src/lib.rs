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

use sgx_types::*;
use sgx_tcrypto::*;
// use sgx_trts::memeq::ConsttimeMemEq;
use std::vec::Vec;
// use std::ptr;
use std::string::String;
use std::string::ToString;
use std::io::{self, Write};
use std::slice;
use std::collections::HashMap;
use blake2::{Blake2b, Digest};
// use sgx_tcrypto::{SgxRsaPrivKey, SgxRsaPubKey};
// use std::fs;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // let mod_size: i32 = 256;
    // let exp_size: i32 = 4;
    // let mut n: Vec<u8> = vec![0_u8; mod_size as usize];
    // let mut d: Vec<u8> = vec![0_u8; mod_size as usize];
    // let mut e: Vec<u8> = vec![1, 0, 1, 0];
    // let mut p: Vec<u8> = vec![0_u8; mod_size as usize / 2];
    // let mut q: Vec<u8> = vec![0_u8; mod_size as usize / 2];
    // let mut dmp1: Vec<u8> = vec![0_u8; mod_size as usize / 2];
    // let mut dmq1: Vec<u8> = vec![0_u8; mod_size as usize / 2];
    // let mut iqmp: Vec<u8> = vec![0_u8; mod_size as usize / 2];

    // let privkey = SgxRsaPrivKey::new();
    // let pubkey = SgxRsaPubKey::new();

    
    // let result1 = pubkey.create(mod_size,
    //                            exp_size,
    //                            n.as_slice(),
    //                            e.as_slice());
    // let result2 = privkey.create(mod_size,
    //                             exp_size,
    //                             e.as_slice(),
    //                             p.as_slice(),
    //                             q.as_slice(),
    //                             dmp1.as_slice(),
    //                             dmq1.as_slice(),
    //                             iqmp.as_slice());


    // print!("{:?}",result1);
    // print!("{:?}",result2);

    let ecc_handle = SgxEccHandle::new();
    let _result = ecc_handle.open();
    let (prv_k, pub_k) = ecc_handle.create_key_pair().unwrap();
    print!("{:?}", prv_k);

    let mut hasher = Blake2b::new();
    let data = "";
    hasher.input(data);
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

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    sgx_status_t::SGX_SUCCESS
}

