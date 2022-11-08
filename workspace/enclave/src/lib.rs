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
extern crate rulinalg;

use sgx_types::*;
use ndarray::Array;
use ndarray::prelude::*;
use ndarray::stack;
use std::iter::FromIterator;
use rulinalg::matrix::Matrix;
use rulinalg::matrix::BaseMatrix;

fn prepare_pcd_matrix(lidar: *const f32, points_num:usize) -> Matrix::<f32> {
    let mut pcd = Matrix::<f32>::zeros(points_num, 3);
    let mut index: usize = 0;
    while index < points_num * 3 {
        unsafe {
            pcd[[index / 3, index % 3]] = *lidar.offset(index as isize) as f32;
        };
        index += 1;
    }
    return pcd;
}

fn prepare_retptr(pcd: Matrix::<f32>, retptr: *mut f32) {
    let mut idx: usize = 0;
    let elem_num: usize = pcd.rows() * pcd.cols();
    while idx < elem_num {
        unsafe {
            *retptr.offset(idx as isize) = pcd[[idx / 4, idx % 4]];
        };
        idx += 1;
    }
}

#[no_mangle]
pub extern "C" fn process_lidar(lidar: *const f32, points_num: usize, retptr: *mut f32) -> sgx_status_t {
    // Load lidar image into ndarray structure
    let pcd = prepare_pcd_matrix(lidar, points_num);
    println!("a {:?}, {:?}", pcd.rows(), pcd.cols());

    let c = Matrix::<f32>::ones(points_num, 1) * 0.2;
    let cpcd = pcd.hcat(&c);
    println!("b {:?}, {:?}", cpcd.rows(), cpcd.cols());

    prepare_retptr(cpcd, retptr);
    sgx_status_t::SGX_SUCCESS
}