import asyncio
import websockets

from player_command import PlayerCommand

connections = set([])

async def handler(websocket, path):
    async for frame in websocket:
        if type(frame) is bytes:
            print("<< ({} bytes of binary data)", len(frame))
            # ToDo: Parse player ID

            # ToDo: Send audio to Julle's amazing AI backend
            
            # ToDo: Send player command to Rust game backend

            # ToDo: Send back to player the detected command to be shown
            # await websocket.send(command.to_string())

        elif type(frame) is str:
            # Player key presses excepted in format such as:
            # 
            # `2up`
            # 
            # where:
            # - frame[0] = player ID as string (numeric 0-9)
            # - frame[1..] = pressed key as string (character)
            try:
                player_id = int(frame[0])
                command = PlayerCommand.from_key_press(frame[1:])
                # ToDo: Send player command to Rust game backend
                print(f"<< {command} by player {player_id}")
                await websocket.send(command.__str__())
            except ValueError:
                print("<< (failed to parse player command):" + frame)

asyncio.get_event_loop().run_until_complete(websockets.serve(handler, 'localhost', 3012))
asyncio.get_event_loop().run_forever()