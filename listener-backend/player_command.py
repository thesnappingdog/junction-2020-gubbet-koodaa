from enum import Enum

class PlayerCommand(Enum):
    # Key press commands
    UP = 'up'
    DOWN = 'down'
    LEFT = 'left'
    RIGHT = 'right'
    # Audio commands
    RUN = 'RUN'

    # def __str__(self):
    #     # ToDo: Custom parsing to string
    #     return self.__str__()

    @classmethod
    def from_key_press(cls, text):
        if text == 'up':
            return cls.UP
        elif text == 'down':
            return cls.DOWN
        elif text == 'left':
            return cls.LEFT
        elif text == 'right':
            return cls.RIGHT
        else:
            return ValueError