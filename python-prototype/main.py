import argparse
import numpy as np
import json
import time
from ground_segmentation import groundSegmentation
from transform_pcd import transformPcd2WorldFrame, downsamplePcd, anomalyDetection, xy2AABBCls
from viz import generatePcdColor, generatePcdListColor, PcdVisualizer

def parseArguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--config_file_path', default='configs/dp-stitch.json', type=str)
    args = parser.parse_args()
    with open(args.config_file_path) as f:
        config = json.load(f)
    return config

def readBoxCoords(file_path, delimiter, voxel_size):
    # box_coords = np.loadtxt(file_path, delimiter)
    box_coords = []
    with open(file_path) as f:
        lines = f.readlines()
        for i in range(1000):
            box_coord = [float(v) for v in lines[i].split(delimiter)]
            print(box_coord)
            box_coords.append(box_coord)
    box_coords = np.array(box_coords)
    object_aabb_cls_lst = []
    for box_coord in box_coords:
        object_aabb_cls = xy2AABBCls(voxel_size, box_coord[0], box_coord[1], 2)
        object_aabb_cls_lst.append(object_aabb_cls)
    return object_aabb_cls_lst

def main():
    start_t = time.time()

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
    if config['pure_display']:
        object_aabb_cls_lst = readBoxCoords(config['box_file']['file_path'], config['box_file']['delimiter'], config['voxel_size'])
        for object_aabb_cls in object_aabb_cls_lst:
            visualizer.addAABBCls(object_aabb_cls)

        pcd = np.concatenate(pcds, axis=0)
        xyz, color = generatePcdColor(pcd, 0.5)
        visualizer.addXyz(xyz, color)
        visualizer.addFrame()
    elif not config['pure_display'] and config['stitch']:
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

    end_t = time.time()
    print(f'time consumed {end_t - start_t}')
    visualizer.show()

if __name__ == '__main__':
    main()