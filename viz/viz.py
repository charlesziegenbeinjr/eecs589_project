import numpy as np
import open3d as o3d
import plotly.graph_objects as go

def main():
    cloud = o3d.io.read_point_cloud("../test/0.pcd") # Read the point cloud
    xyz = np.loadtxt('../test/lidar.txt', delimiter=',')
    colors = np.zeros_like(xyz)
    dists = np.linalg.norm(xyz[:,:2], axis=1)
    colors[:,2] = dists / np.max(dists)
    print(dists.shape)
    print(xyz.shape)
    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(xyz)
    pcd.colors = o3d.utility.Vector3dVector(colors)
    o3d.visualization.draw_geometries([pcd]) # Visualize the point cloud     

if __name__ == '__main__':
    main()