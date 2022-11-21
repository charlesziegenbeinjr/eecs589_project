import numpy as np
import open3d as o3d
import plotly.graph_objects as go

def generatePcdColor(pcd, coeff):
    xyz, c = pcd[:, :3], pcd[:, 3]
    colors = np.ones_like(xyz)
    colors[:, :2] *= coeff
    colors[:, 2] = c
    return xyz, colors

def generatePcdListColor(pcds):
    xyz_list = []
    color_list = []
    for idx, pcd in enumerate(pcds):
        xyz, color = generatePcdColor(pcd, ((idx + 1) * 2 / 3) / len(pcds))
        xyz_list.append(xyz)
        color_list.append(color)
    xyzs = np.concatenate(xyz_list, axis=0)
    colors = np.concatenate(color_list, axis=0)
    return xyzs, colors

def vizArray(xyz, colors):
    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(xyz)
    pcd.colors = o3d.utility.Vector3dVector(colors)
    o3d.visualization.draw_geometries([pcd]) # Visualize the point pcd   

if __name__ == '__main__':
    main()