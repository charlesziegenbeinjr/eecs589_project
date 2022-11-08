import numpy as np
import pdb
from viz import vizArray

def segmentPoints(points, param, z_threshold):
    xy, z = points[:, :2], points[:, 2].reshape(-1, 1)
    xy = np.concatenate((xy, np.ones((points.shape[0], 1))), axis=1)
    z_plane = xy @ param
    z_diff = (z - z_plane).reshape(-1)
    ground_points = points[z_diff < z_threshold]
    object_points = points[z_diff >= z_threshold]
    return ground_points, object_points

def checkInlierNumBatch(points, param, dist_threshold):
    xy, z = points[:, :2], points[:, 2].reshape(-1, 1)
    xy = np.concatenate((xy, np.ones((points.shape[0], 1))), axis=1)
    z_plane = xy @ param
    z_diff = np.abs(z_plane - z)
    inlier_num = z_diff[z_diff < dist_threshold].size
    return inlier_num

def checkInlierNum(points, param, dist_threshold, batch_size):
    inlier_num = 0
    for batch_idx in range(points.shape[0] // batch_size):
        l = batch_idx * batch_size
        r = min((batch_idx + 1) * batch_size, points.shape[0])
        batch = points[l:r]
        inlier_num_n = checkInlierNumBatch(batch, param, dist_threshold)
        inlier_num += inlier_num_n
        # print(f'batch {batch_idx} / {points.shape[0] // batch_size} in num {inlier_num_n}')

    return inlier_num

def fitPlane(points):
    assert points.shape[0] == 3
    A, b = points[:, :2], points[:, 2].reshape(-1, 1)
    A = np.concatenate((A, np.ones((3, 1))), axis=1)
    x = np.linalg.inv(A) @ b
    return x

def ransac(points, dist_threshold, max_iteraton, batch_size):
    best_param = None
    max_inlier_num = -np.inf
    for _ in range(max_iteraton):
        current_indices = np.random.choice(points.shape[0], 
                                            size=3, 
                                            replace=False)  
        current_points = points[current_indices, :]
        current_param = fitPlane(current_points)
        current_inlier_num = checkInlierNum(points, current_param, dist_threshold, batch_size)
        if current_inlier_num > max_inlier_num:
            max_inlier_num = current_inlier_num
            best_param = current_param
    print(f'final max num {max_inlier_num}')
    return best_param

def downsamplePoints(points, remain_num, dist_threshold):
    dists = np.linalg.norm(points, axis=1)
    filtered_points = points[dists < dist_threshold]
    return filtered_points[:min(remain_num, filtered_points.shape[0])]

def main():
    points = np.loadtxt('../test/lidar.txt', delimiter=',')
    print(points.shape)
    points = downsamplePoints(points, 50000, 50)
    print(points.shape, points[:, 2].max(), points[:, 2].min())
    param = ransac(points, 0.1, 200, 50)

    ground_points, object_points = segmentPoints(points, param, 0.25)

    ground_colors = np.ones_like(ground_points) * 0.5
    ground_colors[:, 0] = 0.95
    object_colors = np.ones_like(object_points) * 0.5
    object_colors[:, 2] = 0.95
    print(ground_points.shape, ground_points[:, 2].max(), ground_points[:, 2].min())

    points = np.concatenate((ground_points, object_points), axis=0)
    colors = np.concatenate((ground_colors, object_colors), axis=0)

    vizArray(points, colors)

if __name__ == '__main__':
    main()