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
extern crate blake2;
extern crate hex;

use sgx_types::*;
use std::vec::*;
use std::num::*;
use rulinalg::matrix::{BaseMatrix, Matrix, MatrixSlice, MatrixSliceMut};
use rand::Rng;

use std::slice;
use blake2::{Blake2b, Digest};
use hex::encode;
use std::convert::TryInto;

pub extern "C" fn say_something(lidar: *const u8, points_num: usize, returned_hash: &mut [u8;64]) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(lidar, points_num) };
    println!("Resulting Str_Slice Length: {:?}", str_slice.len());
    
    let mut hasher = Blake2b::new();
    hasher.input(str_slice);
    
    let nonconvert_hash = hasher.result();
    let hash: [u8; 64] = nonconvert_hash.as_slice().try_into().expect("Wrong Length");
    
    // println!("Returned Hash {:?}", returned_hash[2]);
    
    let encoded_hash = encode(hash);
    // let encoded_hash = hash.as_slice();
    *returned_hash = hash;
     
    sgx_status_t::SGX_SUCCESS
}

fn prepare_pcd_matrix(lidar_ptr: *const f32, points_num:usize) -> Matrix::<f32> {
    let mut pcd = Matrix::<f32>::zeros(points_num, 3);
    let mut index: usize = 0;
    while index < points_num * 3 {
        unsafe {
            pcd[[index / 3, index % 3]] = *lidar_ptr.offset(index as isize) as f32;
        };
        index += 1;
    }
    return pcd;
}

fn prepare_lidar_pose_matrix(lidar_pose_ptr: *const f32) -> Matrix::<f32> {
    let mut lidar_pose = Matrix::<f32>::zeros(6, 1);
    let mut index: usize = 0;
    while index < 6 {
        unsafe {
            lidar_pose[[index, 0]] = *lidar_pose_ptr.offset(index as isize) as f32;
        };
        index += 1;
    }
    return lidar_pose;
}

// fn prepare_retptr(pcd: Matrix::<f32>, retptr: *mut f32) {
//     let mut idx: usize = 0;
//     let elem_num: usize = pcd.rows() * pcd.cols();
//     while idx < elem_num {
//         unsafe {
//             *retptr.offset(idx as isize) = pcd[[idx / 4, idx % 4]];
//         };
//         idx += 1;
//     }
// }

fn prepare_retptr(box_coords: Matrix::<f32>, retptr: *mut f32) {
    let mut idx: usize = 0;
    let elem_num: usize = box_coords.rows() * box_coords.cols();
    while idx < elem_num {
        unsafe {
            *retptr.offset(idx as isize) = box_coords[[idx / 2, idx % 2]];
        };
        idx += 1;
    }
}

fn pretty_print_matrix(mat: Matrix::<f32>) {
    println!("================================");
    for i in 0..mat.rows() {
        for j in 0.. mat.cols() {
            print!("{:?}  ", mat[[i, j]]);
        }
        print!("\n---\n");
    }
    println!("================================");
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

fn check_inlier_num(pcd: Matrix::<f32>, param: Matrix::<f32>, z_threshold: f32, use_abs_metric: bool) -> (Matrix::<f32>, i32) {
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
        let mut z_diff = z[[row_idx, 0]] - z_plane[[0, 0]];
        if use_abs_metric {
            z_diff = z_diff.abs();
        }

        if z_diff < z_threshold {
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
        let (current_pcd_cls, current_inlier_num) = check_inlier_num(pcd.clone(), current_param.clone(), z_threshold, true);
        if current_inlier_num > max_inlier_num {
            best_param = current_param;
            max_inlier_num = current_inlier_num;
            best_pcd_cls = current_pcd_cls;
        }
        // println!("epoch {:?} cur {:?} best {:?}", epoch, current_inlier_num, max_inlier_num);
    }
    return (best_param, max_inlier_num);
} 

fn filter_pcd_ground(pcd: Matrix::<f32>, pcd_cls: Matrix::<f32>, inlier_num: i32) -> Matrix::<f32> {
    let size = inlier_num as usize;
    let mut filtered_pcd = Matrix::<f32>::zeros(size, 3);

    let mut row_idx: usize = 0;
    let mut filtered_row_idx: usize = 0;    
    for row in pcd.row_iter() {
        let point = (*row).into_matrix();
        if pcd_cls[[row_idx, 0]] == 0f32 {
            filtered_pcd[[filtered_row_idx, 0]] = point[[0, 0]];
            filtered_pcd[[filtered_row_idx, 1]] = point[[0, 1]];
            filtered_pcd[[filtered_row_idx, 2]] = point[[0, 2]];
            filtered_row_idx += 1;
        }
        row_idx += 1;
    }
    return filtered_pcd;
} 

fn ground_segmentation(pcd: Matrix::<f32>) -> Matrix::<f32> {
    let (best_param, max_inlier_num) = ransac(pcd.clone(), 0.1, 100);
    let (pcd_cls, ground_points_num) = check_inlier_num(pcd.clone(), best_param, 0.1, false);
    let filtered_pcd = filter_pcd_ground(pcd.clone(), pcd_cls.clone(), ground_points_num);
    return filtered_pcd;
}

fn rotation_matrix_x(angle_in_degrees: f32) -> Matrix::<f32> {
    let theta = angle_in_degrees.to_radians();
    let s = theta.sin();
    let c = theta.cos();
    let M = Matrix::new(3, 3, vec![1.0, 0.0, 0.0,
                                   0.0, c,   -s,
                                   0.0, s,    c]);
    return M;
}

fn rotation_matrix_y(angle_in_degrees: f32) -> Matrix::<f32> {
    let theta = angle_in_degrees.to_radians();
    let s = theta.sin();
    let c = theta.cos();
    let M = Matrix::new(3, 3, vec![c,   0.0,   s,
                                   0.0, 1.0,   0.0,
                                   -s,  0.0,   c]);
    return M;
}

fn rotation_matrix_z(angle_in_degrees: f32) -> Matrix::<f32> {
    let theta = angle_in_degrees.to_radians();
    let s = theta.sin();
    let c = theta.cos();
    let M = Matrix::new(3, 3, vec![c,  -s, 0.0,
                                   s,   c,  0.0,
                                   0.0, 0.0, 1.0]);
    return M;
}

fn euler_values_2_matrix(angle_x_in_degrees: f32, angle_y_in_degrees: f32, angle_z_in_degrees: f32) -> Matrix::<f32> {
    let R_x = rotation_matrix_x(angle_x_in_degrees);
    let R_y = rotation_matrix_x(angle_y_in_degrees);
    let R_z = rotation_matrix_x(angle_z_in_degrees);

    let R_3 = R_z * R_y * R_x;
    let R = Matrix::new(4, 4, vec![R_3[[0, 0]], R_3[[0, 1]], R_3[[0, 2]], 0.0,
                                   R_3[[1, 0]], R_3[[1, 1]], R_3[[1, 2]], 0.0,
                                   R_3[[2, 0]], R_3[[2, 1]], R_3[[2, 2]], 0.0,
                                   0.0,         0.0,         0.0,         1.0]); 
    return R;
}

fn translation_values_2_matrix(tx: f32, ty: f32, tz: f32) -> Matrix::<f32> {
    let M = Matrix::new(4, 4, vec![1.0,  0.0,  0.0, tx,
                                   0.0,  1.0,  0.0, ty,
                                   0.0,  0.0,  1.0, tz,
                                   0.0,  0.0,  0.0, 1.0]); 
    return M;
}

fn lidar_pose_2_matrix(lidar_pose: Matrix::<f32>) -> Matrix::<f32> {
    let translation_vec = MatrixSlice::from_matrix(&lidar_pose, [0, 0], 3, 1).into_matrix();
    let tranlation_mat = translation_values_2_matrix(translation_vec[[0, 0]], translation_vec[[1, 0]], translation_vec[[2, 0]]);

    let angles_in_degree = MatrixSlice::from_matrix(&lidar_pose, [3, 0], 3, 1).into_matrix();
    let rotation_mat = euler_values_2_matrix(angles_in_degree[[0, 0]], angles_in_degree[[1, 0]], angles_in_degree[[2, 0]]);   
    
    let T_l2w = tranlation_mat * rotation_mat;
    return T_l2w;
}

fn transform_pcd_2_world_frame(pcd: Matrix::<f32>, lidar_pose: Matrix::<f32>) -> Matrix::<f32> {
    let ones = Matrix::<f32>::ones(pcd.rows(), 1);
    let pcd1 = pcd.hcat(&ones);

    let T_l2w = lidar_pose_2_matrix(lidar_pose);

    let pcd1_w = (T_l2w * pcd1.transpose()).transpose();
    let pcd_w = MatrixSlice::from_matrix(&pcd1_w, [0, 0], pcd.rows(), 3).into_matrix();

    return pcd_w;
}

fn pad_with_value(pcd: Matrix::<f32>, value: f32) -> Matrix::<f32> {
    let ones = Matrix::<f32>::ones(pcd.rows(), 1) * value;
    let padded_pcd = pcd.hcat(&ones);
    return padded_pcd;
}

fn getRanges(lidar_poses: Vec<Matrix::<f32>>, x_dist_threshold: f32, y_dist_threshold: f32) -> Vec<Matrix::<f32>> {
    let aabb = Matrix::new(4, 3, vec![x_dist_threshold,  y_dist_threshold,   0f32,
                                      -x_dist_threshold, y_dist_threshold,   0f32,
                                      -x_dist_threshold, -y_dist_threshold,  0f32,
                                      x_dist_threshold,  -y_dist_threshold,  0f32]);  
    let padded_aabb = pad_with_value(aabb, 1f32).transpose();
    let mut AABBs = Vec::new();
    for i in 0..lidar_poses.len() {
        let T_l2w = lidar_pose_2_matrix(lidar_poses[i].clone());
        let mut AABB = (T_l2w * padded_aabb.clone()).transpose();
        AABB = MatrixSlice::from_matrix(&AABB, [0, 0], AABB.rows(), 2).into_matrix();
        AABBs.push(AABB.clone());
    }
    return AABBs;
}

fn get_AABB_min_max(AABBs: Vec<Matrix::<f32>>) -> (f32, f32, f32, f32) {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (AABBs[0][[0, 0]], AABBs[0][[0, 0]], AABBs[0][[0, 1]], AABBs[0][[0, 1]]);
    for i in 0..AABBs.len() {
        for r in 0..AABBs[0].rows() {
            if AABBs[i][[r, 0]] < x_min {
                x_min = AABBs[i][[r, 0]];
            }
            if AABBs[i][[r, 0]] > x_max {
                x_max = AABBs[i][[r, 0]];
            }
            if AABBs[i][[r, 1]] < y_min {
                y_min = AABBs[i][[r, 1]];
            }
            if AABBs[i][[r, 1]] < y_min {
                y_min = AABBs[i][[r, 1]];
            }                                    
        }
    }
    return (x_min, x_max, y_min, y_max);
}

fn check_point_inside_rec(xp: f32, yp:f32, AABB: Matrix::<f32>) -> bool {
    // pretty_print_matrix(AABB.clone());
    let edges = [
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0]
    ];
    for i in 0..4 {
        let (idx1, idx2) = (edges[i][0], edges[i][1]);
        let (x1, y1) = (AABB[[idx1, 0]], AABB[[idx1, 1]]);
        let (x2, y2) = (AABB[[idx2, 0]], AABB[[idx2, 1]]);       
        let D = (x2 - x1) * (yp - y1) - (xp - x1) * (y2 - y1);
        // println!("i {:?} x1 {:?} y1 {:?} x2 {:?} y2 {:?} xp {:?} yp {:?} D {:?}", i, x1, y1, x2, y2, xp, yp, D);
        // println!("D {:?}", D);
        if D < 0f32 {
            return false;
        }
    }
    return true;
}

fn xy_2_voxel_index(voxel_size: f32, x_min: f32, y_min: f32, x: f32, y: f32) -> (usize, usize) {
    let i = ((x - x_min) / voxel_size) as usize;
    let j = ((y - y_min) / voxel_size) as usize;
    return (i, j);
}

fn voxel_index_2_xy(voxel_size: f32, x_min: f32, y_min: f32, i: usize, j: usize) -> (f32, f32) {
    let x = (i as f32 * voxel_size) + x_min;
    let y = (j as f32 * voxel_size) + y_min;
    return (x, y);
}

fn pcd_2_voxel_map(pcd: Matrix::<f32>, voxel_size: f32, point_count_threshold: f32,
                   x_min: f32, x_max: f32, y_min: f32, y_max: f32, 
                   AABB: Matrix::<f32>) -> Matrix::<f32> {
    let (mut row_num, mut col_num) = xy_2_voxel_index(voxel_size, x_min, y_min, x_max, y_max);
    row_num += 1;
    col_num += 1;
    let mut voxel_map = Matrix::<f32>::zeros(row_num, col_num);
    let mut point_count = Matrix::<f32>::zeros(row_num, col_num);

    for r in 0..pcd.rows() {
        let (x, y) = (pcd[[r, 0]], pcd[[r, 1]]);
        let (i, j) = xy_2_voxel_index(voxel_size, x_min, y_min, x, y);
        
        if !(check_point_inside_rec(x, y, AABB.clone())) {
            continue;
        }
        else if point_count[[i, j]] > point_count_threshold * (pcd.rows() as f32) {
            voxel_map[[i, j]] = 2f32;
        }
        else {
            voxel_map[[i, j]] = 1f32;
            point_count[[i, j]] += 1f32;
        }
    }
    return voxel_map;
}

fn pcds_2_voxel_maps(pcds: Vec<Matrix::<f32>>, voxel_size: f32, point_count_threshold: f32,
                     x_min: f32, x_max: f32, y_min: f32, y_max: f32, 
                     AABBs: Vec<Matrix::<f32>>) -> Vec<Matrix::<f32>> {
    let mut voxel_maps = Vec::new();
    for k in 0..pcds.len() {
        let voxel_map = pcd_2_voxel_map(pcds[k].clone(), voxel_size, point_count_threshold,
                                        x_min, x_max, y_min, y_max, AABBs[k].clone());
        voxel_maps.push(voxel_map.clone());
    }
    return voxel_maps;
}

fn check_proximity(voxel_maps: Vec<Matrix::<f32>>, indices: Vec<usize>, center_i: usize, center_j: usize, radius: i32) -> bool {
    for idx in indices.iter() {
        let k = *idx;
        let (int_li, int_hi) = ((center_i as i32) - radius, (center_i as i32) + radius);
        let (int_lj, int_hj) = ((center_j as i32) - radius, (center_j as i32) + radius);
        for i in int_li..int_hi {
            for j in int_lj..int_hj {
                if !( i > 0 && i < (voxel_maps[k].rows() as i32) && j > 0 && j < (voxel_maps[k].cols() as i32) ) {
                    continue;
                }
                else if voxel_maps[k][[i as usize, j as usize]] == 2f32 {
                    return true;
                }
            }
        }
    }
    return false;
}

fn compare(voxel_maps: Vec<Matrix::<f32>>, voxel_size: f32, x_min: f32, y_min: f32, AABBs: Vec<Matrix::<f32>>) -> Matrix::<f32> {
    let mut box_num = 0usize;
    let mut box_coords = Matrix::<f32>::zeros(voxel_maps.len() * voxel_maps[0].rows() * voxel_maps[0].cols(), 2);
    let mut debug = 0;
    let mut debug1 = 0;

    for mi in 0..voxel_maps.len() {
        for i in 0..voxel_maps[mi].rows() {
            for j in 0..voxel_maps[mi].cols() {
                if voxel_maps[mi][[i, j]] == 2f32 {
                    let (x, y) = voxel_index_2_xy(voxel_size, x_min, y_min, i, j);
                    // box_coords[[box_num, 0]] = x;
                    // box_coords[[box_num, 1]] = y;                    
                    // box_num += 1;
                    debug1 += 1;
                    println!("=================");
                    println!("one {:?}", debug1);                    

                    let mut indices = Vec::new();
                    for mj in 0..voxel_maps.len() {
                        if mj == mi {
                            continue;
                        }
                        else if !(check_point_inside_rec(x, y, AABBs[mj].clone())) {
                            debug += 1;
                            println!("{:?}", debug);
                            continue;
                        }
                        else {
                            indices.push(mj);
                        }
                    }
                    if indices.len() > 0 {
                        println!("in");
                        if !(check_proximity(voxel_maps.clone(), indices, i, j, 5)) {
                            println!("true");
                            box_coords[[box_num, 0]] = x;
                            box_coords[[box_num, 1]] = y;                    
                            box_num += 1;                            
                        }
                    }
                }
            }
        }
    }
    box_coords = MatrixSlice::from_matrix(&box_coords, [0, 0], box_num, 2).into_matrix();
    return box_coords;
}

fn anomaly_detection(pcds: Vec<Matrix::<f32>>, lidar_poses: Vec<Matrix::<f32>>,
                     x_dist_threshold: f32, y_dist_threshold: f32, 
                     voxel_size: f32, point_count_threshold: f32) -> Matrix::<f32> {
    let mut AABBs = getRanges(lidar_poses, x_dist_threshold, y_dist_threshold);
    let (x_min, x_max, y_min, y_max) = get_AABB_min_max(AABBs.clone());
    let voxel_maps = pcds_2_voxel_maps(pcds.clone(), voxel_size, point_count_threshold,
                                       x_min, x_max, y_min, y_max, AABBs.clone());
    let box_coords = compare(voxel_maps, voxel_size, x_min, y_min, AABBs);
    return box_coords;
}

#[no_mangle]
pub extern "C" fn process_lidar(lidar1: *const f32, points_num1: usize, lidar_pose1: *const f32,
                                lidar2: *const f32, points_num2: usize, lidar_pose2: *const f32,
                                retptr: *mut f32) -> sgx_status_t {
    // Load lidar image into ndarray structure
    // ---------------------------------------------------------------
    let pcd1 = prepare_pcd_matrix(lidar1, points_num1);
    let lidar_pose1 = prepare_lidar_pose_matrix(lidar_pose1);
    let pcd2 = prepare_pcd_matrix(lidar2, points_num2);
    let lidar_pose2 = prepare_lidar_pose_matrix(lidar_pose2);

    let pcd1_w = transform_pcd_2_world_frame(pcd1.clone(), lidar_pose1.clone());
    let pcd2_w = transform_pcd_2_world_frame(pcd2.clone(), lidar_pose2.clone());
    
    let mut filtered_pcd1_w = ground_segmentation(pcd1_w);
    let mut filtered_pcd2_w = ground_segmentation(pcd2_w);
    // ---------------------------------------------------------------

    let mut pcds = Vec::new();
    pcds.push(filtered_pcd1_w.clone());
    pcds.push(filtered_pcd2_w.clone());
    let mut lidar_poses = Vec::new();
    lidar_poses.push(lidar_pose1.clone());
    lidar_poses.push(lidar_pose2.clone());

    let box_coords = anomaly_detection(pcds, lidar_poses, 30f32, 10f32, 0.5, 0.00005);

    // let padded_filtered_pcd1_w = pad_with_value(filtered_pcd1_w.clone(), 0f32);
    // let padded_filtered_pcd2_w = pad_with_value(filtered_pcd2_w.clone(), 1f32);
    // let cpcd = padded_filtered_pcd1_w.vcat(&padded_filtered_pcd2_w);
    // let box_coords = Matrix::<f32>::ones(200, 2) * 3.1f32;

    prepare_retptr(box_coords, retptr);
    sgx_status_t::SGX_SUCCESS
}