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
use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
    fn process_lidar(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
                     lidar1: *const f32, points_num1: usize, lidar_pose1: *const f32,
                     lidar2: *const f32, points_num2: usize, lidar_pose2: *const f32,  
                     retptr: *const f32) -> sgx_status_t;
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

fn parse_lidar_pose(file_path: &str) -> [f32; 6] {
    let s = fs::read_to_string(file_path).unwrap();
    let mut lidar_pose: [f32; 6] = [0.0; 6];
    let tokens: Vec<&str> = s.split(" ").collect();
    for (i, token) in tokens.iter().enumerate() {
        lidar_pose[i] = token.parse().unwrap();
    }
    return lidar_pose;
}

fn parse_lidar(file_path: &str) -> Vec<[f32; 3]> {
    let s = fs::read_to_string(file_path).unwrap();
    let mut lidar = Vec::new();
    let lines: Vec<&str> = s.split("\n").collect();
    for line in lines.iter() {
        let xyz_str: Vec<&str> = line.split(" ").collect();
        let mut xyz: [f32; 3] = [0.0; 3];
        for (j, n) in xyz_str.iter().enumerate() {
            if n.len() == 0 {
                break;
            }
            if j >= 3 {
                break;
            }
            xyz[j] = n.parse().unwrap();
        }
        lidar.push(xyz)
    }
    return lidar
}

fn create_file(file_path: &str) -> std::io::Result<()> {
    let mut f = File::create(file_path)?;
    Ok(())
}

fn write_2_lidar_text(file_path: &str, arr: &mut[f32]) {
    create_file(file_path);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    for line_idx in 0..arr.len() / 4 {
        let string = arr[line_idx * 4].to_string() + ","
                    + &arr[line_idx * 4 + 1].to_string() + ","
                    + &arr[line_idx * 4 + 2].to_string() + ","
                    + &arr[line_idx * 4 + 3].to_string();
        if let Err(e) = writeln!(file, "{}", string) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn write_2_xy_text(file_path: &str, arr: &mut[f32]) {
    create_file(file_path);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    for line_idx in 0..arr.len() / 2 {
        let string = arr[line_idx * 2].to_string() + ","
                    + &arr[line_idx * 2 + 1].to_string();
        if let Err(e) = writeln!(file, "{}", string) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn read_lidar_info(pcd_file_path: &str, lidar_pose_file_path: &str) -> ([f32; 180000], [f32; 6], usize) {
    let lidar_pose: [f32; 6] = parse_lidar_pose(lidar_pose_file_path);
    let lidar_vector: Vec<[f32; 3]> = parse_lidar(pcd_file_path);
    let points_num = lidar_vector.len();

    let mut lidar: [f32; 180000] = [0.0; 180000];
    for (i, point) in lidar_vector.iter().enumerate() {
        for j in 0..3 {
            lidar[i * 3 + j] = point[j];
        }
    }

    return (lidar, lidar_pose, points_num);
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

    let (lidar1, lidar_pose1, points_num1) = read_lidar_info("../opv2v/2005_000069_anomaly.txt",
                                                        "../opv2v/2005_000069_lidar_pose.txt");    
    let (lidar2, lidar_pose2, points_num2) = read_lidar_info("../opv2v/2014_000069.txt",
                                                        "../opv2v/2014_000069_lidar_pose.txt");            

    const retsize:usize = 540000 + (180000 / 3);
    let mut retarr: [f32; retsize] = [2.0; retsize];
    println!("allocated {}", retarr.len());
    // println!("11 {:?}", retarr);
    let result = unsafe {
        process_lidar(enclave.geteid(),
                      &mut retval,
                      lidar1.as_ptr() as *const f32,
                      points_num1,
                      lidar_pose1.as_ptr() as *const f32,
                      lidar2.as_ptr() as *const f32,
                      points_num2,
                      lidar_pose2.as_ptr() as *const f32,
                      retarr.as_ptr() as *const f32)
    };
    println!("after result");

    // unsafe {
    //     write_2_lidar_text("../test/output.txt", &mut retarr);
    //     println!("after write");
    // }
    unsafe {
        write_2_xy_text("../test/output.txt", &mut retarr);
        println!("after write");
    }    
    
    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }
    println!("[-] ECALL Enclave Failed {}!", result.as_str());
    enclave.destroy();
}