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

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate ndarray;

use sgx_types::*;
use ndarray::Array;


#[no_mangle]
pub extern "C" fn process_lidar(lidar: *const f32, points_num: usize, retptr: *mut f32) -> sgx_status_t {
    // Load lidar image into ndarray structure.
    let mut pcd = Array::<f32, _>::zeros((points_num, 3));
    let mut index: usize = 0;
    while index < points_num * 3 {
        unsafe {
            pcd[[index / 3, index % 3]] = *lidar.offset(index as isize) as f32;
        };
        index += 1;
    }
    println!("Enclave received lidar image {:?}", pcd.shape());

    // Point cloud preprocessing (voxelization to reduce point count).

    // Ground detection.

    // Object segmentation.

    // Point cloud reduction (remain only object and ground points).
    
    // Occupancy map generation.

    //
    let mut index1: usize = 0;
    while index1 < 10 {
        unsafe {
            let x:f32 = index1 as f32;
            *retptr.offset(index1 as isize) = x;
        };
        index1 += 1;
    }
    //

    sgx_status_t::SGX_SUCCESS
}