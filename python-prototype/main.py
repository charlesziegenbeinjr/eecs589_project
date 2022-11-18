import argparse
import numpy as np
from ground_segmentation import groundSegmentation
from transform_pcd import transformPcd2WorldFrame
from viz import generatePcdColor, vizArray

def parseArguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('--pcd_file_path', nargs='?', default='/home/ruohuali/Desktop/output.txt', type=str)
    parser.add_argument('--pcd_delimiter', nargs='?', default=',', type=str)
    parser.add_argument('--lidar_pose_file_path', nargs='?', default='/home/ruohuali/Desktop/output.txt', type=str)
    parser.add_argument('--lidar_pose_delimiter', nargs='?', default=',', type=str)    
    parser.add_argument('--ground_segmentation', nargs='?', default=0, type=int)
    parser.add_argument('--transform', nargs='?', default=0, type=int)
    args = parser.parse_args()
    args.ground_segmentation = args.ground_segmentation == 1
    args.transform = args.transform == 1
    return args

def main():
    args = parseArguments()
    pcd = np.loadtxt(args.pcd_file_path, delimiter=args.pcd_delimiter)
    points = groundSegmentation(pcd) if args.ground_segmentation else pcd
    if args.transform:
        lidar_pose = np.loadtxt(args.lidar_pose_file_path, delimiter=args.lidar_pose_delimiter)
        points = transformPcd2WorldFrame(points, lidar_pose, np.array([0, 0, 0]))
    
    xyz, colors = generatePcdColor(points, 0.5)
    vizArray(xyz, colors)

if __name__ == '__main__':
    main()