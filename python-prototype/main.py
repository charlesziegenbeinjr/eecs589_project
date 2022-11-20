import argparse
import numpy as np
import json
from ground_segmentation import groundSegmentation, downsamplePcd
from transform_pcd import stitchPcds
from viz import generatePcdColor, vizArray

def parseArguments():
    # parser = argparse.ArgumentParser()
    # parser.add_argument('--pcd_file_path', nargs='?', default='/home/ruohuali/Desktop/output.txt', type=str)
    # parser.add_argument('--pcd_delimiter', nargs='?', default=',', type=str)
    # parser.add_argument('--lidar_pose_file_path', nargs='?', default='/home/ruohuali/Desktop/output.txt', type=str)
    # parser.add_argument('--lidar_pose_delimiter', nargs='?', default=',', type=str)    
    # parser.add_argument('--ground_segmentation', nargs='?', default=0, type=int)
    # parser.add_argument('--transform', nargs='?', default=0, type=int)
    # args = parser.parse_args()
    # args.ground_segmentation = args.ground_segmentation == 1
    # args.transform = args.transform == 1

    parser = argparse.ArgumentParser()
    parser.add_argument('--config_file_path', default='configs/default.json', type=str)
    args = parser.parse_args()
    with open(args.config_file_path) as f:
        config = json.load(f)
    return config

def main():
    config = parseArguments()
    pcds = []
    lidar_poses = []
    for fc in config['files']:
        pcd = np.loadtxt(fc['pcd_file_path'], delimiter=fc['pcd_file_delimiter'])
        pcd = groundSegmentation(pcd) if fc['ground_segmentation'] else pcd
        if config['stitch']:
            lidar_pose = np.loadtxt(fc['lidar_pose_file_path'], delimiter=fc['lidar_pose_file_delimiter'])
            lidar_poses.append(lidar_pose)
        pcds.append(pcd)

    if config['stitch']:
        xyzs, colors = stitchPcds(pcds, lidar_poses)
        if config['downsample']:
            pcd = downsamplePcd(pcd, 50, 10)        
        vizArray(xyzs, colors)
    else:
        for pcd in pcds:
            xyz, color = generatePcdColor(pcd, 0.5)
            vizArray(xyz, color)

if __name__ == '__main__':
    main()