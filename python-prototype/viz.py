import numpy as np
import open3d as o3d

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

class PcdVisualizer():
    def __init__(self):
        self.geometries = []

    def addXyz(self, xyz, colors):
        pcd = o3d.geometry.PointCloud()
        pcd.points = o3d.utility.Vector3dVector(xyz)
        pcd.colors = o3d.utility.Vector3dVector(colors)
        self.geometries.append(pcd)

    def addLine(self, p1, p2, color=[1, 0, 0]):
        p1, p2 = list(p1), list(p2)
        points = [p1, p2]
        lines = [[0, 1]]
        colors = [color for i in range(len(lines))]
        line_set = o3d.geometry.LineSet()
        line_set.points = o3d.utility.Vector3dVector(points)
        line_set.lines = o3d.utility.Vector2iVector(lines)
        line_set.colors = o3d.utility.Vector3dVector(colors)
        self.geometries.append(line_set)

    def addPolygon(self, vertices, color=[0, 0, 0.5]):
        for i in range(len(vertices)):
            if i == len(vertices) - 1:
                p1, p2 = vertices[i], vertices[0]
            else:
                p1, p2 = vertices[i], vertices[i + 1]
            self.addLine(p1, p2, color=color)

    def addFrame(self, origin=[0, 0, 0]):
        mesh_frame = o3d.geometry.TriangleMesh.create_coordinate_frame(
            size=5, origin=origin)
        self.geometries.append(mesh_frame)

    def show(self):
        viewer = o3d.visualization.Visualizer()
        viewer.create_window()
        for geometry in self.geometries:
            viewer.add_geometry(geometry)
        # opt = viewer.get_render_option()
        # opt.show_coordinate_frame = True
        # opt.background_color = np.asarray([0.5, 0.5, 0.5])
        viewer.run()
        viewer.destroy_window()

if __name__ == '__main__':
    pass    