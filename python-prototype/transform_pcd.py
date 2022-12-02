import numpy as np
from scipy.spatial.transform import Rotation   

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

def transformPcd2WorldFrame(pcd, pose, world_center=np.zeros((3,))):
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

def downsamplePcdAlong1Axis(pcd, dists, dist_threshold):
    filtered_pcd = pcd[dists < dist_threshold]
    return filtered_pcd

def downsamplePcd(pcd, x_dist_threshold, y_dist_threshold):
    x_dists = np.abs(pcd[:, 0])
    filtered_pcd = downsamplePcdAlong1Axis(pcd, x_dists, x_dist_threshold)
    y_dists = np.abs(filtered_pcd[:, 1])
    filtered_pcd = downsamplePcdAlong1Axis(filtered_pcd, y_dists, y_dist_threshold)
    return filtered_pcd

def getRanges(lidar_poses, x_dist_threshold, y_dist_threshold):
    aabb = np.array([[x_dist_threshold, y_dist_threshold, 0],
                     [-x_dist_threshold, y_dist_threshold, 0],
                     [-x_dist_threshold, -y_dist_threshold, 0],
                     [x_dist_threshold, -y_dist_threshold, 0]]) # 4 x 3
    aabb1 = np.concatenate((aabb, np.ones((aabb.shape[0], 1))), axis=1).T # 4 x 4 c

    AABBs = []
    for lidar_pose in lidar_poses:
        T_l2w = lidarPose2Matrix(lidar_pose)
        AABB = (T_l2w @ aabb1).T # 4 x 4 r
        AABB = AABB[:, :2] # 4 x 2 r
        AABBs.append(AABB)
    AABBs = np.array(AABBs)
    print(AABBs.shape)
    return AABBs # N x 4 x 2

def checkPointInsideRec(yp, xp, AABB):
    edges = [
        (AABB[0], AABB[1]),
        (AABB[1], AABB[2]),
        (AABB[2], AABB[3]),
        (AABB[3], AABB[0])
    ]
    for edge in edges:
        v1, v2 = edge
        y1, x1 = v1
        y2, x2 = v2
        D = (x2 - x1) * (yp - y1) - (xp - x1) * (y2 - y1)
        print(v1, v2, D)
        print(x1, y1)
        print(x2, y2)
        print(xp, yp)
        if D < 0:
            return False
    return True

def pcds2VoxelMaps(pcds, voxel_size, point_count_threshold, x_min, x_max, y_min, y_max, AABBs):
    voxel_maps = []
    for pcd in pcds:
        voxel_map = np.zeros(((x_max - x_min) // voxel_size, (y_max - y_min) // voxel_size))
        point_count = np.zeros_like(voxel_map)
        for k, xyzc in enumerate(pcd):
            x, y = xyzc[:2]
            i, j = (x - x_min) // voxel_size, (y - y_min) // voxel_size
            point_count[i, j] += 1
            if not checkPointInsideRec(x, y, AABBs[k]):
                continue
            elif point_count[i, j] > point_count_threshold * pcd.shape[0]:
                voxel_map[i, j] = 2
            else:
                voxel_map[i, j] = 1
        voxel_maps.append(voxel_map)
    return voxel_maps

def anomalyDetection(pcds, lidar_poses, x_dist_threshold, y_dist_threshold, voxel_size, point_count_threshold):
    AABBs = getRanges(lidar_poses, x_dist_threshold, y_dist_threshold)
    x_min, x_max = np.min(AABBs[:, :, 0]), np.max(AABBs[:, :, 0])
    y_min, y_max = np.min(AABBs[:, :, 1]), np.max(AABBs[:, :, 1])

if __name__ == '__main__':
    print(checkPointInsideRec(2.4, 1.1, np.array([[4, 1],
                                                [2, 1],
                                                [2, 2],
                                                [4, 2]])))