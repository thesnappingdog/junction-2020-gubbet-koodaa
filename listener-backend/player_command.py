from enum import Enum

class PlayerCommand(Enum):
    NICK = 'nick:'
    # Key press commands
    UP = 'up'
    DOWN = 'down'
    LEFT = 'left'
    RIGHT = 'right'
    # Audio commands
    RUN = 'run'
    # Connection events
    CONNECT = 'connect'
    DISCONNECT = 'disconnect'

    def to_maze_event(self, name):
        return f"{name}:{self.__str__()}"

    @classmethod
    def from_key_press(cls, text):
        if text == 'up':
            return (cls.UP, None)
        elif text == 'down':
            return (cls.DOWN, None)
        elif text == 'left':
            return (cls.LEFT, None)
        elif text == 'right':
            return (cls.RIGHT, None)
        elif text.startswith('nick:'):
            return (cls.NICK, text[5:])
        else:
            return (None, None)