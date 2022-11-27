import argparse
import numpy as np
import json
from ground_segmentation import groundSegmentation
from transform_pcd import transformPcd2WorldFrame, downsamplePcd
from viz import generatePcdColor, generatePcdListColor, PcdVisualizer

def parseArguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--config_file_path', default='configs/dp-stitch.json', type=str)
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
        if config['downsample']:
            x_dist_threshold, y_dist_threshold = config['downsample_dist_threshold']
            pcd = downsamplePcd(pcd, x_dist_threshold, y_dist_threshold)      
        if config['stitch']:
            lidar_pose = np.loadtxt(fc['lidar_pose_file_path'], delimiter=fc['lidar_pose_file_delimiter'])
            lidar_poses.append(lidar_pose)
            pcd = transformPcd2WorldFrame(pcd, lidar_pose)
        pcds.append(pcd)

    visualizer = PcdVisualizer()
    if config['stitch']:
        xyzs, colors = generatePcdListColor(pcds) 
        visualizer.addXyz(xyzs, colors)
        visualizer.show()
    else:
        for pcd in pcds:
            xyz, color = generatePcdColor(pcd, 0.5)
            visualizer.addXyz(xyz, color)
            visualizer.addFrame()
            # visualizer.addLine([0, 0, 1], [1, 1, 1])
            visualizer.show()

if __name__ == '__main__':
    main()