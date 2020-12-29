import socket
from twisted.internet import reactor
from twisted.web.server import Site
from twisted.web.static import File
from autobahn.twisted.websocket import listenWS

from server import ServerFactory, ServerProtocol, send_maze
from player_command import PlayerCommand
from player_map import PlayerMap
from audio_control.controller import audio_controller

global PLAYER_MAP
PLAYER_MAP = PlayerMap()

def run():
    # Websocket server
    # ServerFactory = ServerFactory
    factory = ServerFactory("ws://localhost:3012")
    factory.protocol = ServerProtocol
    factory.onRegister = handleRegister
    factory.onUnregister = handleUnregister
    factory.onTextMessage = handleTextMessage
    factory.onBinaryMessage = handleBinaryMessage

    #Audio controller
    audio_controller()

    listenWS(factory)

    # Optional HTTP server
    webdir = File(".")
    web = Site(webdir)
    reactor.listenTCP(80, web)
    reactor.run()


# Handler functions

def handleRegister(client, factory):
    factory.broadcast(f"[{client.peer}] joined da geim!")

def handleUnregister(client, factory):
    uuid = PLAYER_MAP.get_name_by_peer(client.peer)
    # ToDo: Tell maze dis playa left
    # maze_communicator.disconnected(uuid)
    factory.broadcast(f"[{client.peer}] left da geim :(")

def handleBinaryMessage(peer, payload, factory):
    name = PLAYER_MAP.get_name_by_peer(peer)
    if name == None:
        print(f"Can't map {peer} to registered player name")
        return
    
    print(f"[{name}] >> ({len(payload)} bytes of data)")

    # ToDo: Use this instead of short circuited controller
    command = None
    if command == None:
        return

    # Send player command to Rust game backend
    event = command.to_maze_event(name)
    send_maze(event)

    # ToDo: Send back to player the detected command to be shown
    # factory.send(peer, command.__str__())

def handleTextMessage(peer, msg, factory):
    command, text = PlayerCommand.from_key_press(msg)
    if command == None:
        print(f"[{peer}] >> \"" + msg + "\" (failed to parse key press command)")
        return
    
    if command == PlayerCommand.NICK:
        name = text
        old_name = PLAYER_MAP.pop_peer(peer)
        if old_name != None:
            print(f"[{peer}] changed name from '{old_name}' to '{name}' -- this is not handled with maze at the moment!")
            pass
        PLAYER_MAP.register(peer, name)
        print(f"[{peer}] name set to '{name}'!")
        
        # Initialize player for the nick in Rust game backend
        event = PlayerCommand.CONNECT.to_maze_event(name)
        print(event)
        send_maze(event)

        # Send back confirmation to player that nickname was successfully noticed
        factory.send(peer, f"Your nickname is set to {text}")
    else:
        name = PLAYER_MAP.get_name_by_peer(peer)
        if name == None:
            print(f"Can't map {peer} to registered player name")
            return
        
        print(f"[{name}] >> {command}")
        
        # Send player command to Rust game backend
        event = command.to_maze_event(name)
        # event = f"{name}:{command.__str__()}"
        send_maze(event)
        
        # Send back confirmation to player that key press was successfully noticed
        factory.send(peer, command.__str__())

if __name__ == "__main__":
    run()
