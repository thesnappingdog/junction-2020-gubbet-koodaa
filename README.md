# Voice controlled maze

![alt text](https://github.com/hietalajulius/junction-2020/blob/main/cover.png)
### Running the code:
Run below steps in separate terminal windows

#### Game backend
1. [Install rust](https://www.rust-lang.org/tools/install)
2. `cd maze && cargo run --release`

#### Controller backend
1. `cd listener-backend`
2. Install dependencies from `requirements.txt` (dependent on conda/venv setup)
3. Run `python3 server.py`
4. The model is sensitive to noise, so keep the computer on a table in a relatively quiet place
5. Available voice commands: ["up", "down", "left", "right"]
