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
        pcd = groundSegmentation(pcd) if fc['ground_segmentation'] else pcd
        if config['stitch']:
            x_dist_threshold, y_dist_threshold = config['dist_threshold']
        if config['downsample']:
            pcd = downsamplePcd(pcd, x_dist_threshold, y_dist_threshold)  
            pcd = pcd[pcd[:, -1] == 0]    
        if config['stitch']:
            lidar_pose = np.loadtxt(fc['lidar_pose_file_path'], delimiter=fc['lidar_pose_file_delimiter'])
            lidar_poses.append(lidar_pose)
            pcd = transformPcd2WorldFrame(pcd, lidar_pose)
        pcds.append(pcd)

    visualizer = PcdVisualizer()
    if config['stitch']:
        if config['anomaly_detection']:
            voxel_size = config['voxel_size']
            point_count_threshold = config['point_count_threshold']
            object_aabb_vertices, AABBs = anomalyDetection(pcds, lidar_poses, x_dist_threshold, y_dist_threshold, voxel_size, point_count_threshold)
            for object_aabb_vertice in object_aabb_vertices:
                c = object_aabb_vertice['color']
                color = [0, 0, 0]
                color[c] = 1
                visualizer.addPolygon(object_aabb_vertice['vertice'], color)
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
            # visualizer.addPolygon([[6, 6, 0],
            #                       [1, 6, 0],
            #                       [1, 2, 0],
            #                       [6, 2, 0]])
            visualizer.show()

if __name__ == '__main__':
    main()