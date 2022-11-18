import numpy as np
import open3d as o3d
import plotly.graph_objects as go

def generatePcdColor(pcd, coeff):
    xyz, c = pcd[:, :3], pcd[:, 3]

    colors = np.ones_like(xyz)
    colors[:, :2] *= coeff
    colors[:, 2] = c
    return xyz, colors

def vizArray(xyz, colors):
    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(xyz)
    pcd.colors = o3d.utility.Vector3dVector(colors)
    o3d.visualization.draw_geometries([pcd]) # Visualize the point pcd   

if __name__ == '__main__':
    main()