# receiver.py

import socket

HOST = "127.17.0.3"  # Standard loopback interface address (localhost)
PORT = 8080  # Port to listen on (non-privileged ports are > 1023)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        print(f"Connected by {addr}")
        tmp = conn.recv(4)
        lidar_size = int.from_bytes(tmp, byteorder='little')
        tmp = conn.recv(4)
        pose_size = int.from_bytes(tmp, byteorder='little')
        lidar = conn.recv(lidar_size)
        pose = conn.recv(pose_size)

        #do pcd_algorithm
        reply = "GOOD"
        s.send(reply.encode('utf-8'))