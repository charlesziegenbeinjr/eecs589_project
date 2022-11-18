import numpy as np
from scipy.spatial.transform import Rotation   
from viz import vizArray, generatePcdColor
from ground_segmentation import groundSegmentation

def euler2Matrix(angles):
    r = Rotation.from_euler("xyz", angles, degrees=True)
    rotation_matrix = r.as_matrix()
    return rotation_matrix

def lidarPose2Matrix(pose):
    xyz, angles = pose[:3], pose[3:]

    rotation_matrix = np.identity(4)
    rotation_matrix[:3, :3] = euler2Matrix(angles)

    translation_matrix = np.identity(4)
    translation_matrix[:3, 3] = xyz

    matrix = translation_matrix @ rotation_matrix
    return matrix

def transformPcd2WorldFrame(pcd, pose, world_center):
    xyz, c = pcd[:, :3], pcd[:, 3]
    xyz1 = np.concatenate((xyz, np.ones((xyz.shape[0], 1))), axis=1)

    T_w2l = lidarPose2Matrix(pose)
    # T_l2w = np.linalg.inv(T_w2l)
    T_l2w = T_w2l.copy()
    xyz1 = (T_l2w @ (xyz1.T)).T 

    xyz = xyz1[:, :3]
    xyz += world_center
    pcd = np.concatenate((xyz, c.reshape(-1, 1)), axis=1)
    return pcd

def stitchTwoPcd(pcd1, pcd2, pose1, pose2, world_center):
    # pcd2_w = pcd1.copy()
    pcd1_w = transformPcd2WorldFrame(pcd1, pose1, world_center)
    pcd2_w = transformPcd2WorldFrame(pcd2, pose2, world_center)
    xyz1, colors1 = generatePcdColor(pcd1_w, 0.5)
    xyz2, colors2 = generatePcdColor(pcd2_w, 0.75)
    xyz = np.concatenate((xyz1, xyz2), axis=0)
    colors = np.concatenate((colors1, colors2), axis=0)
    vizArray(xyz, colors)

if __name__ == '__main__':
    pcd1 = np.loadtxt('/home/ruohuali/Desktop/eecs589_project/opv2v/2005_000069.txt', delimiter=' ')
    # points1 = groundSegmentation(pcd1)
    lidar_pose1 = np.loadtxt('/home/ruohuali/Desktop/eecs589_project/opv2v/2005_000069_lidar_pose.txt', delimiter=' ')
    
    pcd2 = np.loadtxt('/home/ruohuali/Desktop/eecs589_project/opv2v/2014_000069.txt', delimiter=' ')
    # points2 = groundSegmentation(pcd2)
    lidar_pose2 = np.loadtxt('/home/ruohuali/Desktop/eecs589_project/opv2v/2014_000069_lidar_pose.txt', delimiter=' ')
    
    stitchTwoPcd(pcd1, pcd2, lidar_pose1, lidar_pose2, np.array([0, 0, 0]))
