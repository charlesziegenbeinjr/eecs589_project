# sender.py

import socket
import timeit

start = timeit.default_timer()
HOST = "127.17.0.3"  # The server's hostname or IP address
PORT = 8081  # The port used by the server
REC = "127.17.0.4"

lidar_path = "./anomaly.txt"
pose_path = "./pose.txt"
f = open(lidar_path, 'rb')
lidar = f.read()
f.close()
lidar_size = len(lidar)
f = open(pose_path, 'rb')
pose = f.read()
f.close()
pose_size = len(pose)
print(lidar)
print(pose)
print(lidar_size)
print(pose_size)
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(lidar_size.to_bytes(4, 'little', signed=False))
    s.sendall(pose_size.to_bytes(4, 'little', signed=False))
    s.sendall(lidar)
    s.sendall(pose)
    print("Data Sent")
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((REC, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        reply = conn.recv( 1024 ).decode( 'utf-8' )
        print("Received ", str(reply))

stop = timeit.default_timer()
print('Time: ', stop - start)  
