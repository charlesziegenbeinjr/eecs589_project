# sender.py

import socket
import timeit

start = timeit.default_timer()
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
    s.sendall(bytes(lidar_size))
    s.sendall(bytes(pose_size))
    s.sendall(lidar)
    s.sendall(pose)
    print("Data Sent")
    reply = s.recv( 1024 ).decode( 'utf-8' )
    print("Received ", str(reply))

stop = timeit.default_timer()
print('Time: ', stop - start)  
