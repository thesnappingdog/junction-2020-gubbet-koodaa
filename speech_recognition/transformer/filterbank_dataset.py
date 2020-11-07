import sys, os
import torch
import librosa
import numpy as np
import pandas as pd
from torch import Tensor
from scipy.io import wavfile
from torchvision import transforms
from torch.utils.data import DataLoader
from torch.utils.data.dataset import Dataset
import torchaudio


class FilterBankDataset(Dataset):
    def __init__(self, transform=None, mode="train"):
        # setting directories for data
        self.transform = transform
        self.df = pd.read_csv("../digits.csv")
        self.n_fft = 400.0
        self.vector_size = 40
        self.mel_bins = 24
    def __len__(self):
        return len(self.df) 

    def __getitem__(self, idx):
        filename = self.df["filename"][idx]
        
        waveform, sample_rate = torchaudio.load("../../../free-spoken-digit-dataset/recordings/"+filename)
        
        frame_length = self.n_fft / sample_rate * 1000.0
        frame_shift = frame_length / 2.0
        params = {
            "channel": 0,
            "dither": 0.0,
            "window_type": "hanning",
            "frame_length": frame_length,
            "frame_shift": frame_shift,
            "remove_dc_offset": True,
            "round_to_power_of_two": True,
            "sample_frequency": sample_rate,
            "num_mel_bins": self.mel_bins
        }
        
        data = torchaudio.compliance.kaldi.fbank(waveform, **params)

        if self.transform is not None:
            data = self.transform(data)
        
        
        if data.size()[0] > self.vector_size:
            data = data[:self.vector_size,:]
        else:
            base_vector = torch.zeros((self.vector_size, self.mel_bins))
            base_vector[:data.size()[0],:] = data
            data = base_vector
            
            
        label = self.df["label"][idx]

        return data, label


def collate(batch):
    mels = 24
    bptt = 40
    data = torch.stack([item[0] for item in batch], dim=1)
    labels = torch.tensor([item[1] for item in batch])
    return data, labels