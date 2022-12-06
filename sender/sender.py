# sender.py

import socket

HOST = "127.17.0.3"  # The server's hostname or IP address
PORT = 8080  # The port used by the server

lidar_path = ""
pose_path = ""
f = open(lidar_path, 'rb')
lidar = f.read()
f.close()
lidar_size = len(lidar)
f = open(pose_path, 'rb')
pose = f.read()
f.close()
pose_size = len(pose)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.send(bytes(lidar_size))
    s.send(bytes(pose_size))
    s.send(lidar)
    s.send(pose)

print("Data Sent")