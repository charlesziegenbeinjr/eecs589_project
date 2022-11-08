import numpy as np
import open3d as o3d
import plotly.graph_objects as go

def vizArray(xyz, colors):
    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(xyz)
    pcd.colors = o3d.utility.Vector3dVector(colors)
    o3d.visualization.draw_geometries([pcd]) # Visualize the point cloud   

def main():
    cloud = np.loadtxt('/home/ruohuali/Desktop/output.txt', delimiter=',')
    xyz, c = cloud[:, :3], cloud[:, 3]
    print(c.shape)
    c = np.clip(c * 20, 0, 1)
    colors = np.ones_like(xyz) * 0.1
    colors[:, 2] = c
    # colors = np.zeros_like(xyz)
    # dists = np.linalg.norm(xyz[:,:2], axis=1)
    # colors[:,2] = dists / np.max(dists)
    # print(dists.shape)
    print(xyz.shape)
    vizArray(xyz, colors)

if __name__ == '__main__':
    main()