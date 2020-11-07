from autobahn.twisted.websocket import WebSocketServerProtocol

# Article: https://medium.com/python-in-plain-english/identify-websocket-clients-with-autobahn-twisted-and-python-3f90b4c135d4
# Source: https://stackoverflow.com/questions/29951718/autobahn-sending-user-specific-and-broadcast-messages-from-external-application
class ServerProtocol(WebSocketServerProtocol):
    def onOpen(self):
        self.factory.register(self)

    def onConnect(self, request):
        print("Client connecting: {} connecting".format(request.peer))

    def onMessage(self, payload, isBinary):
        if isBinary:
            self.factory.onBinaryMessage(self.peer, payload, self.factory)
        else:
            msg = payload.decode("utf-8")
            self.factory.onTextMessage(self.peer, msg, self.factory)

    def broadcast(self, payload):
        self.factory.broadcast(payload)

    def connectionLost(self, reason):
        WebSocketServerProtocol.connectionLost(self, reason)
        self.factory.unregister(self)
