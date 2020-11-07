from autobahn.twisted.websocket import WebSocketServerFactory
from autobahn.exception import Disconnected

def defaultOnRegister(client, factory):
    print("Registered client {}".format(client.peer))

def defaultOnUnregister(client, factory):
    print("Unregistered client {}".format(client.peer))

def defaultOnTextMessage(peer, msg, factory):
    print(f"{peer} >> {msg}")

def defaultOnBinaryMessage(peer, payload, factory):
    print(f"{peer} >> ({len(payload)} bytes of data)")

class ServerFactory(WebSocketServerFactory):
    def __init__(self, url):
        WebSocketServerFactory.__init__(self, url)
        self.clients = dict()
        self.onRegister = defaultOnRegister
        self.onUnregister = defaultOnUnregister
        self.onTextMessage = defaultOnTextMessage
        self.onBinaryMessage = defaultOnTextMessage

    def register(self, client):
        if client not in self.clients:
            self.clients[client.peer] = client
            self.onRegister(client, self)

    def unregister(self, client):
        if client in self.clients:
            self.clients.pop(client.peer, None)
            self.onUnregister(client, self)

    def broadcast(self, msg):
        for c in self.clients.values():
            try:
                c.sendMessage(msg.encode('utf-8'))
            except Disconnected:
                self.unregister(c)

    def send(self, peer, msg):
        c = self.clients.get(peer)
        if c != None:
            try:
                c.sendMessage(msg.encode('utf-8'))
            except Disconnected:
                self.unregister(c)
