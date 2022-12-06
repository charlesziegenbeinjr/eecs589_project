# receiver.py

import socket
import time
HOST = "127.17.0.3"  # Standard loopback interface address (localhost)
PORT = 8081  # Port to listen on (non-privileged ports are > 1023)
REC = "127.17.0.4"
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        print(f"Connected by {addr}")
        tmp = conn.recv(4)
        lidar_size = int.from_bytes(tmp, 'little', signed=False)
        tmp = conn.recv(4)
        pose_size = int.from_bytes(tmp, 'little', signed=False)
        lidar = conn.recv(lidar_size)
        pose = conn.recv(pose_size)
        print(lidar_size)
        print(pose_size)
        #do pcd_algorithm
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    time.sleep(0.01)
    s.connect((REC, PORT))
    reply = "GOOD"
    s.send(reply.encode('utf-8'))