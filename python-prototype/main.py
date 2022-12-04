import argparse
import numpy as np
import json
from ground_segmentation import groundSegmentation
from transform_pcd import transformPcd2WorldFrame, downsamplePcd, anomalyDetection
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
        if config['stitch']:
            lidar_pose = np.loadtxt(fc['lidar_pose_file_path'], delimiter=fc['lidar_pose_file_delimiter'])
            lidar_poses.append(lidar_pose)
            pcd = transformPcd2WorldFrame(pcd, lidar_pose)        
        if fc['ground_segmentation']:
            pcd = groundSegmentation(pcd)
        if config['stitch']:
            x_dist_threshold, y_dist_threshold = config['dist_threshold']
        if config['downsample']:
            pcd = downsamplePcd(pcd, x_dist_threshold, y_dist_threshold)      
        pcds.append(pcd)

    visualizer = PcdVisualizer()
    if config['stitch']:
        if config['anomaly_detection']:
            voxel_size = config['voxel_size']
            point_count_threshold = config['point_count_threshold']
            object_aabb_cls_lst, AABBs = anomalyDetection(pcds, lidar_poses, x_dist_threshold, y_dist_threshold, voxel_size, point_count_threshold)
            for object_aabb_cls in object_aabb_cls_lst:
                visualizer.addAABBCls(object_aabb_cls)
            for aabb in AABBs:
                aabb = np.concatenate((aabb, np.ones((aabb.shape[0], 1))), axis=1)
                visualizer.addPolygon(aabb, [1, 0, 1])
        xyzs, colors = generatePcdListColor(pcds) 
        visualizer.addXyz(xyzs, colors)
        visualizer.show()
    else:
        for pcd in pcds:
            xyz, color = generatePcdColor(pcd, 0.5)
            visualizer.addXyz(xyz, color)
            visualizer.addFrame()
            # visualizer.addLine([0, 0, 1], [1, 1, 1])
            visualizer.addPolygon([[-15, -5, 0],
                                  [-10, -10, 0],
                                  [-10, -5, 0],
                                  [-15, -10, 0]])
            visualizer.show()

if __name__ == '__main__':
    main()