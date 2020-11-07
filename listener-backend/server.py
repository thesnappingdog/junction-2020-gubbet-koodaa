import asyncio
import websockets

text_frame_counter = 0

async def handler(websocket, path):
    async for frame in websocket:
        if type(frame) is bytes:
            print("<< ({} bytes of binary data)", len(frame))
            # ToDo: Parse player ID
            # ToDo: Send audio to Julle's amazing AI backend
        elif type(frame) is str:
            print("<< " + frame)
            await websocket.send(frame)
            print(">> {}", frame)

asyncio.get_event_loop().run_until_complete(websockets.serve(handler, 'localhost', 3012))
asyncio.get_event_loop().run_forever()