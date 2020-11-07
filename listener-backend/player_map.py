class PlayerMap():
    def __init__(self):
        self.map = dict()

    def register(self, peer, name):
        self.map[peer] = name
        print(f"[PlayerMap]: Player {name} at {peer} joined")

    def unregister_name(self, name):
        for _peer, _name in self.map.items():
            if _name == name:
                self.unregister_peer(_peer)

    def pop_peer(self, peer):
        name = self.map.pop(peer, None)
        if name != None:
            print(f"[PlayerMap]: Player {name} at {peer} joined")
            return name
        return None

    def get_name_by_peer(self, peer):
        return self.map.get(peer, None)

    def get_peer_by_name(self, name):
        for _peer, _name in self.map.items():
            if _name == name:
                return self.get(_peer)
        return None
