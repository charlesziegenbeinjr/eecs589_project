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
// specific language governing perM_issions and liM_itations
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
extern crate rand;

use sgx_types::*;
use std::vec::*;
use std::num::*;
use rulinalg::matrix::{BaseMatrix, Matrix, MatrixSlice, MatrixSliceMut};
use rand::Rng;

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

fn sample_ids(n: i32, min:usize, max:usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _i in 0..n {
        vec.push(rng.gen_range(min, max));
    }
    return vec;
}

fn sample_3_points(pcd: Matrix::<f32>) -> Matrix::<f32> {
    let ids = sample_ids(3, 0, pcd.rows());
    let mut three_points = Matrix::<f32>::zeros(3, 3);
    for i in 0..3 {
        let idx = ids[i];
        let point = pcd.row(idx);
        for c in 0..3 {
            three_points[[i, c]] = point[c];
        }
    }
    println!("dfads {:?}", three_points);
    return three_points;
}

fn inverse3x3(M: Matrix::<f32>) -> Matrix::<f32> {
    let m1 = M[[0, 0]];
    let m2 = M[[0, 1]];
    let m3 = M[[0, 2]];

    let m4 = M[[1, 0]];
    let m5 = M[[1, 1]];
    let m6 = M[[1, 2]];

    let m7 = M[[2, 0]];
    let m8 = M[[2, 1]];
    let m9 = M[[2, 2]];

    let determinant = m1*m5*m9 + m4*m8*m3 + m7*m2*m6 - m1*m6*m8 - m3*m5*m7 - m2*m4*m9;
    
    let mut M_i = Matrix::<f32>::zeros(3, 3);
    M_i[[0, 0]] = m5*m9-m6*m8;
    M_i[[0, 1]] = m3*m8-m2*m9;
    M_i[[0, 2]] = m2*m6-m3*m5;

    M_i[[1, 0]] = m6*m7-m4*m9;
    M_i[[1, 1]] = m1*m9-m3*m7;
    M_i[[1, 2]] = m3*m4-m1*m6;

    M_i[[2, 0]] = m4*m8-m5*m7;
    M_i[[2, 1]] = m2*m7-m1*m8;
    M_i[[2, 2]] = m1*m5-m2*m4;

    M_i = M_i / determinant;
    return M_i;
}

fn fit_plane(three_points: Matrix::<f32>) -> Matrix::<f32> {
    let xy = MatrixSlice::from_matrix(&three_points, [0, 0], 3, 2).into_matrix();
    let ones = Matrix::<f32>::ones(3, 1);
    let xy1 = xy.hcat(&ones);
    let z = MatrixSlice::from_matrix(&three_points, [0, 2], 3, 1).into_matrix();
    let xy1_inv = inverse3x3(xy1.clone());
    let param = xy1_inv * z;
    return param;
}

fn check_inlier_num(pcd: Matrix::<f32>, param: Matrix::<f32>, z_threshold: f32) -> (Matrix::<f32>, i32) {
    let xy = MatrixSlice::from_matrix(&pcd, [0, 0], pcd.rows(), 2).into_matrix();
    let ones = Matrix::<f32>::ones(pcd.rows(), 1);
    let xy1 = xy.hcat(&ones);
    let z = MatrixSlice::from_matrix(&pcd, [0, 2], pcd.rows(), 1).into_matrix();
    let mut pcd_cls = Matrix::<f32>::zeros(pcd.rows(), 1);
    let mut inlier_num: i32 = 0;

    let mut row_idx: usize = 0;
    for row in xy1.row_iter() {
        let point = (*row).into_matrix();
        let z_plane = point * param.clone();
        let z_diff = z_plane[[0, 0]] - z[[row_idx, 0]];

        if z_diff.abs() < z_threshold {
            inlier_num += 1;
            pcd_cls[[row_idx, 0]] = 1f32;
        }
        row_idx += 1;
    }

    return (pcd_cls, inlier_num); 
}

fn ransac(pcd: Matrix::<f32>, z_threshold: f32, iteration_num: i32) -> (Matrix::<f32>, i32) {
    let mut best_param = Matrix::<f32>::zeros(3, 1);
    let mut max_inlier_num: i32 = 0;
    let mut best_pcd_cls = Matrix::<f32>::zeros(pcd.rows(), 1);
    for epoch in 0..iteration_num {
        let current_points = sample_3_points(pcd.clone());
        let current_param = fit_plane(current_points);
        let (current_pcd_cls, current_inlier_num) = check_inlier_num(pcd.clone(), current_param.clone(), z_threshold);
        if current_inlier_num > max_inlier_num {
            best_param = current_param;
            max_inlier_num = current_inlier_num;
            best_pcd_cls = current_pcd_cls;
        }
        println!("epoch {:?} cur {:?} best {:?}", epoch, current_inlier_num, max_inlier_num);
    }
    return (best_pcd_cls, max_inlier_num);
} 

#[no_mangle]
pub extern "C" fn process_lidar(lidar: *const f32, points_num: usize, retptr: *mut f32) -> sgx_status_t {
    // Load lidar image into ndarray structure
    let pcd = prepare_pcd_matrix(lidar, points_num);
    println!("a {:?}, {:?}", pcd.rows(), pcd.cols());

    let (best_pcd_cls, max_inlier_num) = ransac(pcd.clone(), 0.1, 100);

    // let c = Matrix::<f32>::ones(points_num, 1) * 0.2;
    let cpcd = pcd.hcat(&best_pcd_cls);
    println!("b {:?}, {:?}", cpcd.rows(), cpcd.cols());

    prepare_retptr(cpcd, retptr);
    sgx_status_t::SGX_SUCCESS
}