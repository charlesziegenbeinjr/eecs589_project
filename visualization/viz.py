import numpy as np
import matplotlib.pyplot as plt

lidar = np.loadtxt('../test/lidar.txt', delimiter=',')
print(lidar.shape)

ax = plt.figure().add_subplot(projection='3d')
y = lidar[:, 0]
x = lidar[:, 1]
z = lidar[:, 2]
# By using zdir='y', the y value of these points is fixed to the zs value 0
# and the (x, y) points are plotted on the x and z axes.
ax.scatter(z, x, y, label='points in (x, z)')

# Make legend, set axes limits and labels
ax.legend()
# ax.set_xlim(0, 1)
# ax.set_ylim(0, 1)
# ax.set_zlim(0, 1)
ax.set_xlabel('X')
ax.set_ylabel('Y')
ax.set_zlabel('Z')

ax.set_xlim(x.min(), x.max())
ax.set_ylim(y.min(), y.max())
ax.set_zlim(z.min(), z.max())
ax.autoscale(False)

# Customize the view angle so it's easier to see that the scatter points lie
# on the plane y=0
# ax.view_init(elev=20., azim=-35, roll=0)
plt.show()