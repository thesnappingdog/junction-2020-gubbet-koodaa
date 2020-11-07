import asyncio
import websockets

counter = 0

async def handler(websocket, path):
    async for message in websocket:
        print(">> " + message)
        await websocket.send(message)
        print("Sent reply #{}: {}", counter, message)

asyncio.get_event_loop().run_until_complete(websockets.serve(handler, 'localhost', 3012))
asyncio.get_event_loop().run_forever()