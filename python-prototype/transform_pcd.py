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
    return AABBs # N x 4 x 2

def checkPointInsideRec(xp, yp, AABB):
    edges = [
        (AABB[0], AABB[1]),
        (AABB[1], AABB[2]),
        (AABB[2], AABB[3]),
        (AABB[3], AABB[0])
    ]
    for edge in edges:
        v1, v2 = edge
        x1, y1 = v1
        x2, y2 = v2
        D = (x2 - x1) * (yp - y1) - (xp - x1) * (y2 - y1)
        if D < 0:
            return False
    return True

def pcds2VoxelMaps(pcds, voxel_size, point_count_threshold, x_min, x_max, y_min, y_max, AABBs):
    voxel_maps = []
    for k, pcd in enumerate(pcds):
        voxel_map = np.zeros(( int((x_max - x_min) // voxel_size) + 1, int((y_max - y_min) // voxel_size) + 1 ))
        point_count = np.zeros_like(voxel_map)
        for xyzc in pcd:
            x, y = xyzc[:2]
            i, j = int((x - x_min) // voxel_size), int((y - y_min) // voxel_size)
            if not checkPointInsideRec(x, y, AABBs[k]):
                continue
            elif point_count[i, j] > point_count_threshold * pcd.shape[0]:
                voxel_map[i, j] = 2
            else:
                voxel_map[i, j] = 1
                point_count[i, j] += 1
        voxel_maps.append(voxel_map)
    return voxel_maps

def voxelIndex2Xy(voxel_size, x_min, y_min, i, j):
    x = (i * voxel_size) + x_min
    y = (j * voxel_size) + y_min
    return x, y

def voxelIndex2AABBCls(voxel_size, x_min, y_min, i, j, cls):
    x, y = voxelIndex2Xy(voxel_size, x_min, y_min, i, j)
    return np.array([[x + voxel_size, y + voxel_size],
                     [x - voxel_size, y + voxel_size],
                     [x - voxel_size, y - voxel_size],
                     [x + voxel_size, y - voxel_size],
                     [cls, 0]])

def checkProximity(voxel_maps, center_i, center_j, radius):
    for voxel_map in voxel_maps:
        for i in range(center_i - radius, center_i + radius):
            for j in range(center_j - radius, center_j + radius):
                if not(0 < i < voxel_map.shape[0] and 0 < j < voxel_map.shape[1]):
                    continue
                elif voxel_map[i, j] == 2:
                    return True
    return False

def compare(voxel_maps, voxel_size, x_min, y_min, AABBs):
    object_aabb_cls_lst = []
    for mi, voxel_map in enumerate(voxel_maps):
        voxel_ids_np = np.where(voxel_map == 2)
        voxel_ids = []
        for vi in range(len(voxel_ids_np[0])):
            i, j = voxel_ids_np[0][vi], voxel_ids_np[1][vi]
            voxel_ids.append((i, j))
            object_aabb_cls = voxelIndex2AABBCls(voxel_size, x_min, y_min, i, j, mi)
            # object_aabb_cls_lst.append(object_aabb_cls)

            other_voxel_maps = []
            x, y = voxelIndex2Xy(voxel_size, x_min, y_min, i, j)
            for mj in range(len(voxel_maps)):
                if mj == mi:
                    continue
                elif not checkPointInsideRec(x, y, AABBs[mj]):
                    continue
                else:
                    other_voxel_maps.append(voxel_maps[mj])
            if len(other_voxel_maps) > 0:
                if not checkProximity(other_voxel_maps, i, j, 5):
                    object_aabb_cls = voxelIndex2AABBCls(voxel_size, x_min, y_min, i, j, -1)
                    object_aabb_cls_lst.append(object_aabb_cls)
    return object_aabb_cls_lst

def anomalyDetection(pcds, lidar_poses, x_dist_threshold, y_dist_threshold, voxel_size, point_count_threshold):
    AABBs = getRanges(lidar_poses, x_dist_threshold, y_dist_threshold)
    # print(AABBs)
    x_min, x_max = np.min(AABBs[:, :, 0]), np.max(AABBs[:, :, 0])
    y_min, y_max = np.min(AABBs[:, :, 1]), np.max(AABBs[:, :, 1])
    voxel_maps = pcds2VoxelMaps(pcds, voxel_size, point_count_threshold, x_min, x_max, y_min, y_max, AABBs)
    object_aabb_vertices = compare(voxel_maps, voxel_size, x_min, y_min, AABBs)
    return object_aabb_vertices, AABBs

if __name__ == '__main__':
    tests = [[5, 10], [3, 8], [3, 4], [5, 4], [4, 1], [2, -1], [-3, 4], [-5, -4]]
    for t in tests:
        print(checkPointInsideRec(t[0], t[1], np.array([[6, 6],
                                                        [1, 6],
                                                        [1, 2],
                                                        [6, 2]])))