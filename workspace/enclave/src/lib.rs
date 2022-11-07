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
use ndarray::prelude::*;
use ndarray::stack;
use std::iter::FromIterator;

fn prepare_retptr(pcd: Array2<f32>, retptr: *mut f32) {
    let mut idx: usize = 0;
    let elem_num: usize = pcd.len();
    while idx < elem_num {
        unsafe {
            *retptr.offset(idx as isize) = pcd[[idx / 4, idx % 4]];
        };
        idx += 1;
    }
    println!("final index {}", idx);
}

fn maximum_1d_f32(arr: Array1<f32>) -> f32 {
	let mut flat_arr = Array::from_iter(arr.iter().cloned());

    let mut cur_max = -f32::INFINITY;
    for ((idx), value) in flat_arr.indexed_iter() {
        if cur_max < *value {
            cur_max = *value;
        }
    }
    return cur_max;
}

fn maximum_3d_i32(arr: Array3<i32>) -> i32 {
	let mut flat_arr = Array::from_iter(arr.iter().cloned());

    let mut cur_max = 0i32;
    for ((idx), value) in flat_arr.indexed_iter() {
        if cur_max < *value {
            cur_max = *value;
        }
    }
    return cur_max;
}

fn minimum_1d_f32(arr: Array1<f32>) -> f32 {
	let mut flat_arr = Array::from_iter(arr.iter().cloned());

    let mut cur_min = f32::INFINITY;
    for ((idx), value) in flat_arr.indexed_iter() {
        if cur_min > *value {
            cur_min = *value;
        }
    }
    return cur_min;
}

fn voxelize_pcd(pcd: Array2<f32>, voxel_size: f32) -> Array3<i32> {
    let ground_thickness: f32 = 0.25;

    let pcd_x = pcd.index_axis(Axis(1), 0).to_owned();
    let min_x: f32 = minimum_1d_f32(pcd_x.clone());
    let max_x: f32 = maximum_1d_f32(pcd_x.clone());
    let x_length: usize = ((max_x - min_x) / voxel_size + 1f32)as usize;
    println!("x {} {} {}", min_x, max_x, x_length);

    let pcd_y = pcd.index_axis(Axis(1), 1).to_owned();
    let min_y: f32 = minimum_1d_f32(pcd_y.clone());
    let max_y: f32 = maximum_1d_f32(pcd_y.clone());
    let y_length: usize = ((max_y - min_y) / voxel_size + 1f32)as usize;
    println!("y {} {} {}", min_y, max_y, y_length);  
    
    let pcd_z = pcd.index_axis(Axis(1), 2).to_owned();
    let min_z: f32 = minimum_1d_f32(pcd_z.clone());
    let max_z: f32 = maximum_1d_f32(pcd_z.clone());
    let z_length: usize = ((max_z - min_z) / voxel_size + 1f32)as usize;
    println!("z {} {} {}", min_z, max_z, z_length);    

    let mut voxels = Array3::<i32>::zeros((x_length, y_length, z_length));

    for point in pcd.outer_iter() {
        if point[2] > min_z + ground_thickness {
            let x_idx: usize = ((point[0] - min_x) / voxel_size) as usize;
            let y_idx: usize = ((point[1] - min_y) / voxel_size) as usize;
            let z_idx: usize = ((point[2] - min_z) / voxel_size) as usize;
            
            voxels[[x_idx, y_idx, z_idx]] += 1i32;
        }
    }
    return voxels;
}

fn segment_pcd(pcd: Array2<f32>, points_num: usize) -> Array2<f32> {
    let voxel_size = 0.25;

    let voxels = voxelize_pcd(pcd.clone(), voxel_size);
    println!("voxels \n{:?}", voxels);
    let mut pcd_cls = Array2::<f32>::zeros((points_num, 1));


    let pcd_x = pcd.index_axis(Axis(1), 0).to_owned();
    let min_x: f32 = minimum_1d_f32(pcd_x.clone());
    let max_x: f32 = maximum_1d_f32(pcd_x.clone());
    let x_length: usize = ((max_x - min_x) / voxel_size + 1f32)as usize;
    println!("x {} {} {}", min_x, max_x, x_length);

    let pcd_y = pcd.index_axis(Axis(1), 1).to_owned();
    let min_y: f32 = minimum_1d_f32(pcd_y.clone());
    let max_y: f32 = maximum_1d_f32(pcd_y.clone());
    let y_length: usize = ((max_y - min_y) / voxel_size + 1f32)as usize;
    println!("y {} {} {}", min_y, max_y, y_length);  
    
    let pcd_z = pcd.index_axis(Axis(1), 2).to_owned();
    let min_z: f32 = minimum_1d_f32(pcd_z.clone());
    let max_z: f32 = maximum_1d_f32(pcd_z.clone());
    let z_length: usize = ((max_z - min_z) / voxel_size + 1f32)as usize;
    println!("z {} {} {}", min_z, max_z, z_length);    

    let max_point_num = maximum_3d_i32(voxels.clone());
    let mut idx: usize = 0;

    for point in pcd.outer_iter() {
        let x_idx: usize = ((point[0] - min_x) / voxel_size) as usize;
        let y_idx: usize = ((point[1] - min_y) / voxel_size) as usize;
        let z_idx: usize = ((point[2] - min_z) / voxel_size) as usize;
        
        println!("{}", voxels[[x_idx, y_idx, z_idx]]);
        pcd_cls[[idx, 0]] = (voxels[[x_idx, y_idx, z_idx]] as f32) / (max_point_num as f32);
        idx += 1;
    }

    // let x = segment_pcd.mean();
    return pcd_cls;
}

#[no_mangle]
pub extern "C" fn process_lidar(lidar: *const f32, points_num: usize, retptr: *mut f32) -> sgx_status_t {
    // Load lidar image into ndarray structure.
    let mut pcd = Array2::<f32>::zeros((points_num, 3));
    let mut index: usize = 0;
    while index < points_num * 3 {
        unsafe {
            pcd[[index / 3, index % 3]] = *lidar.offset(index as isize) as f32;
        };
        index += 1;
    }
    println!("Enclave received lidar image {:?}, {:?}", pcd.shape(), pcd.len());

    let pcd_cls = segment_pcd(pcd.clone(), points_num);  

    println!("pcl cls {:?}", pcd_cls.shape());

    // let mut colors = Array2::<f32>::zeros((points_num, 1));
    let mut cpcd = stack![Axis(1), pcd, pcd_cls];

    println!("Enclave received lidar image {:?}, {:?}", cpcd.shape(), cpcd.len());

    prepare_retptr(cpcd, retptr);
    println!("Enclave return");
    sgx_status_t::SGX_SUCCESS
}