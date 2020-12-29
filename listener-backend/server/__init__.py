import socket
from .factory import ServerFactory
from .protocol import ServerProtocol


def send_maze(event):
    print(event)
    maze_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    maze_socket.connect(('localhost', 8080))
    maze_socket.send(bytes(event, 'utf-8'))
    maze_socket.close()
