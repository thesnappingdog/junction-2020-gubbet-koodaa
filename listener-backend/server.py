from twisted.internet import reactor
from twisted.web.server import Site
from twisted.web.static import File
from autobahn.twisted.websocket import listenWS

from server import ServerFactory, ServerProtocol
from player_command import PlayerCommand
from player_map import PlayerMap

def handleRegister(client, factory):
    # ToDo: Get player UUID for dis guy from de maze!
    uuid = 3

    PLAYER_MAP.register(client.peer, uuid)
    factory.broadcast(f"Player {uuid} joined da geim!")

def handleUnregister(client, factory):
    try:
        uuid = PLAYER_MAP.get_uuid_by_peer(client.peer)
    except ValueError:
        print(f"<< (failed to find player UUID) for unregistering {client.peer}")
    # ToDo: Tell maze dis playa left
    # maze_communicator.player_left(uuid)

    PLAYER_MAP.unregister_peer(client.peer, 3)
    factory.broadcast(f"Player {uuid} left da geim :(")

def handleBinaryMessage(peer, payload, factory):
    uuid = PLAYER_MAP.get_uuid_by_peer(peer)
    if uuid == None:
        print("<< (failed to find player UUID) for binary data")
        return
    print(f"[Player {uuid}] >> ({len(payload)} bytes of data)")

    # ToDo: Send audio bytes to Julle's amazing AI backend
    
    # ToDo: Send player command to Rust game backend

    # ToDo: Send back to player the detected command to be shown
    # factory.send(peer, command.__str__())

def handleTextMessage(peer, msg, factory):
    uuid = PLAYER_MAP.get_uuid_by_peer(peer)
    command = PlayerCommand.from_key_press(msg)
    if uuid == None:
        print(f"[Player ?] >> \"" + msg + "\" (failed to find player UUID)")
        return
    if command == None:
        print(f"[Player {uuid}] >> \"" + msg + "\" (failed to parse player command)")
        return
    # ToDo: Send player command to Rust game backend
    print(f"[Player {uuid}] >> {command}")
    # Send back confirmation to player that key press was successfully noticed
    factory.send(peer, command.__str__())

global PLAYER_MAP
PLAYER_MAP = PlayerMap()

if __name__ == "__main__":
    ServerFactory = ServerFactory
    factory = ServerFactory("ws://localhost:3012")
    factory.protocol = ServerProtocol
    factory.onRegister = handleRegister
    factory.onUnregister = handleUnregister
    factory.onTextMessage = handleTextMessage
    factory.onBinaryMessage = handleBinaryMessage
    listenWS(factory)

    webdir = File(".")
    web = Site(webdir)
    reactor.listenTCP(8080, web)
    reactor.run()