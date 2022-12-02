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
extern crate hex;

use sgx_types::*;

use std::slice;
use blake2::{Blake2b, Digest};
use hex::encode;
use std::convert::TryInto;




#[no_mangle]
pub extern "C" fn say_something(lidar: *const u8, points_num: usize, returned_hash: &mut [u8;64]) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(lidar, points_num) };
    println!("Resulting Str_Slice Length: {:?}", str_slice.len());
    
    let mut hasher = Blake2b::new();
    hasher.input(str_slice);
    
    let nonconvert_hash = hasher.result();
    println!("Result: {:x}", nonconvert_hash);
    println!("Result: {:?}", nonconvert_hash);
    println!("Result Length: {:?}", nonconvert_hash.len());

    
    let hash: [u8; 64] = nonconvert_hash.as_slice().try_into().expect("Wrong Length");
    println!("Result After Conversion: {:?}", hash);
    println!("Result Length: {:?}", hash.len());

    // println!("Returned Hash {:?}", returned_hash[2]);
    
    let encoded_hash = encode(hash);
    println!("Hash In Hex: {:?}", encoded_hash);

    // let encoded_hash = hash.as_slice();

    *returned_hash = hash;
     
    sgx_status_t::SGX_SUCCESS
}

