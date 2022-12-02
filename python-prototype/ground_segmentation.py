import numpy as np

def segmentXyz(xyz, param, z_threshold):
    xy, z = xyz[:, :2], xyz[:, 2].reshape(-1, 1)
    xy = np.concatenate((xy, np.ones((xyz.shape[0], 1))), axis=1)
    z_plane = xy @ param
    z_diff = (z - z_plane).reshape(-1)
    ground_xyz = xyz[z_diff < z_threshold]
    object_xyz = xyz[z_diff >= z_threshold]

    ground_cls = np.ones((ground_xyz.shape[0], 1))
    object_cls = np.zeros((object_xyz.shape[0], 1))
    ground_points = np.concatenate((ground_xyz, ground_cls), axis=1)
    object_points = np.concatenate((object_xyz, object_cls), axis=1)
    points = np.concatenate((ground_points, object_points), axis=0)

    return points

def checkInlierNumBatch(xyz, param, dist_threshold):
    xy, z = xyz[:, :2], xyz[:, 2].reshape(-1, 1)
    xy = np.concatenate((xy, np.ones((xyz.shape[0], 1))), axis=1)
    z_plane = xy @ param
    z_diff = np.abs(z_plane - z)
    inlier_num = z_diff[z_diff < dist_threshold].size
    return inlier_num

def checkInlierNum(xyz, param, dist_threshold, batch_size):
    inlier_num = 0
    for batch_idx in range(xyz.shape[0] // batch_size):
        l = batch_idx * batch_size
        r = min((batch_idx + 1) * batch_size, xyz.shape[0])
        batch = xyz[l:r]
        inlier_num_n = checkInlierNumBatch(batch, param, dist_threshold)
        inlier_num += inlier_num_n
        # print(f'batch {batch_idx} / {xyz.shape[0] // batch_size} in num {inlier_num_n}')

    return inlier_num

def fitPlane(xyz):
    assert xyz.shape[0] == 3
    A, b = xyz[:, :2], xyz[:, 2].reshape(-1, 1)
    A = np.concatenate((A, np.ones((3, 1))), axis=1)
    x = np.linalg.inv(A) @ b
    return x

def ransac(xyz, dist_threshold, max_iteraton, batch_size):
    best_param = None
    max_inlier_num = -np.inf
    for _ in range(max_iteraton):
        current_indices = np.random.choice(xyz.shape[0], 
                                            size=3, 
                                            replace=False)  
        current_xyz = xyz[current_indices, :]
        current_param = fitPlane(current_xyz)
        current_inlier_num = checkInlierNum(xyz, current_param, dist_threshold, batch_size)
        if current_inlier_num > max_inlier_num:
            max_inlier_num = current_inlier_num
            best_param = current_param
    print(f'final max num {max_inlier_num}')
    return best_param

def groundSegmentation(pcd):
    xyz = pcd[:, :3]
    param = ransac(xyz, 0.1, 200, 50)
    points = segmentXyz(xyz, param, 1)
    return points

if __name__ == '__main__':
    pass
