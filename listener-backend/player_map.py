class PlayerMap():
    def __init__(self):
        self.pool = dict()

    def register(self, peer, uuid):
        self.pool[peer] = uuid
        print(f"[PlayerUuidMap]: Player {uuid} at {peer} joined")

    def unregister_uuid(self, uuid):
        for _peer, _uuid in self.pool.items():
            if _uuid == uuid:
                self.unregister_peer(_peer)

    def unregister_peer(self, peer):
        uuid = self.pool.pop(peer, None)
        if uuid != None:
            print(f"[PlayerUuidMap]: Player {uuid} at {peer} joined")

    def get_uuid_by_peer(self, peer):
        return self.pool.get(peer, None)

    def get_peer_by_uuid(self, uuid):
        for _peer, _uuid in self.pool.items():
            if _uuid == uuid:
                return self.get(_peer)
        return None