import numpy as np
import sounddevice as sd
import torch
import torchaudio
from audio_control.transformer_model import TransformerModel
from player_command import PlayerCommand
from server import send_maze
import os


def audio_controller():
    #Register player
    event = PlayerCommand.CONNECT.to_maze_event("Player1")
    print(event)
    send_maze(event)

    #Define recognition model
    bptt = 80
    ntoken_embedding = None
    embedding_size = 24
    nhid = 256
    nlayers = 6
    nhead = 8
    model = TransformerModel(ntoken_embedding, embedding_size, nhead, nhid,
                             nlayers, n_outputs=30, ready_embedding=True, dropout=0.2, bptt=bptt)

    model_dirname = os.path.dirname(__file__)
    model_filename = os.path.join(model_dirname, 'model.mdl')
    model.load_state_dict(torch.load(
        model_filename, map_location=torch.device('cpu')))
    src_mask = model.generate_square_subsequent_mask(80)
    fs = 16000
    sample_rate = fs
    n_fft = 400.0
    frame_length = n_fft / sample_rate * 1000.0
    frame_shift = frame_length / 2.0
    window = np.zeros((15000, 1))
    np_flag = np.array([False])
    params = {
        "channel": 0,
        "dither": 0.0,
        "window_type": "hanning",
        "frame_length": frame_length,
        "frame_shift": frame_shift,
        "remove_dc_offset": True,
        "round_to_power_of_two": True,
        "sample_frequency": sample_rate,
        "num_mel_bins": 24
    }

    #Handle control signals
    def callback(indata, frames, time, status):
        window[:5000, :] = window[5000:10000, :]
        window[5000:10000, :] = window[10000:15000, :]
        window[10000:15000, :] = indata
        if np.max(indata) > 7000 and not np_flag[0]:
            np_flag[0] = True
            waveform = torch.tensor(window, dtype=torch.float32)
            waveform = waveform.t() / torch.max(waveform)
            fbank = torchaudio.compliance.kaldi.fbank(waveform, **params)
            data = torch.zeros((80, 24))
            data[:fbank.shape[0], :] = fbank
            data = data.view((80, 1, 24))
            pred = model(data, src_mask)
            pred_idx = torch.argmax(pred)
            if pred_idx == 0:
                event = PlayerCommand.UP.to_maze_event("Player1")
            elif pred_idx == 1:
                event = PlayerCommand.DOWN.to_maze_event("Player1")
            elif pred_idx == 2:
                event = PlayerCommand.LEFT.to_maze_event("Player1")
            elif pred_idx == 3:
                event = PlayerCommand.RIGHT.to_maze_event("Player1")
            else:
                event = None
            if not event is None:
                print(event)
                send_maze(event)
        else:
            np_flag[0] = False

    with sd.InputStream(channels=1, dtype='int16', blocksize=5000, samplerate=fs, callback=callback):
        while True:
            sd.sleep(1000000)  # Arbitrary
