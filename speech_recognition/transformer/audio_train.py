import torchtext
from torchtext.data.utils import get_tokenizer
from transformer_model import TransformerModel
import time
import torch
import torch.nn as nn
import math
import torchaudio
from filterbank_dataset import FilterBankDataset, collate
from torch.utils.data import DataLoader

if __name__ == "__main__":
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    filename = "../test.mp3"
    waveform, sample_rate = torchaudio.load(filename)
    n_fft = 400.0
    frame_length = n_fft / sample_rate * 1000.0
    frame_shift = frame_length / 2.0

    params = {
        "channel": 0,
        "dither": 0.0,
        "window_type": "hanning",
        "frame_length": frame_length,
        "frame_shift": frame_shift,
        "remove_dc_offset": False,
        "round_to_power_of_two": False,
        "sample_frequency": sample_rate,
        "num_mel_bins": 24
    }
    fbank = torchaudio.compliance.kaldi.fbank(waveform, **params)
    

    def batchify():
        dada1 = fbank.view((fbank.size()[0], 1, fbank.size()[1]))
        dada2 = fbank.view((fbank.size()[0], 1, fbank.size()[1]))
        dada3 = fbank.view((fbank.size()[0], 1, fbank.size()[1]))

        tens = torch.vstack((dada1,dada2,dada3))
        #rint("tens sh", tens.size())
        return tens.to(device)

    batch_size = 30
    bptt = 40
    
    def get_batch():
        data = batchify()
        target = torch.tensor([1,2,3])
        return data, target


    ntoken_embedding = None #len(TEXT.vocab.stoi) # the size of vocabulary # AMOUNT OF COMMANDS
    embedding_size = 24 # embedding dimension
    nhid = 200 # the dimension of the feedforward network model in nn.TransformerEncoder
    nlayers = 2 # the number of nn.TransformerEncoderLayer in nn.TransformerEncoder
    nhead = 2 # the number of heads in the multiheadattention models
    dropout = 0.2 # the dropout value
    model = TransformerModel(ntoken_embedding, embedding_size, nhead, nhid, nlayers, n_outputs=10, ready_embedding=True, dropout=0.2, batch_size=batch_size, bptt=40).to(device)


    criterion = nn.CrossEntropyLoss()
    lr = 5.0 # learning rate
    optimizer = torch.optim.SGD(model.parameters(), lr=lr)
    scheduler = torch.optim.lr_scheduler.StepLR(optimizer, 1.0, gamma=0.95)

    dataloader = DataLoader(
        FilterBankDataset(), 
        batch_size=batch_size,
        collate_fn=collate)


    def train():
        model.train() # Turn on the train mode
        total_loss = 0.
        start_time = time.time()
        src_mask = model.generate_square_subsequent_mask(bptt).to(device)
        for i, batch in enumerate(dataloader):
            data = batch[0]
            targets = batch[1]
            optimizer.zero_grad()
            if data.size(0) != bptt:
                src_mask = model.generate_square_subsequent_mask(data.size(0)).to(device)
            output = model(data, src_mask)
            #print("Pred", torch.argmax(output, dim=1))
            output = model(data, src_mask)
            #print("Targ", targets)
            loss = criterion(output, targets)
            corr = torch.argmax(output, dim=1) == targets
            corr_len = torch.where(corr == True)[0].size()[0]
            #print("corr", corr_len, corr_len/batch_size)
            print(corr_len/batch_size)
            #print("Loss", loss.item(), "\n")
            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), 0.5)
            optimizer.step()

            total_loss += loss.item()


    def evaluate(eval_model, data_source):
        eval_model.eval() # Turn on the evaluation mode
        total_loss = 0.
        ntokens = len(TEXT.vocab.stoi)
        src_mask = model.generate_square_subsequent_mask(bptt).to(device)
        with torch.no_grad():
            for i in range(0, data_source.size(0) - 1, bptt):
                data, targets = get_batch(data_source, i)
                if data.size(0) != bptt:
                    src_mask = model.generate_square_subsequent_mask(data.size(0)).to(device)
                output = eval_model(data, src_mask)
                output_flat = output.view(-1, ntokens)
                total_loss += len(data) * criterion(output_flat, targets).item()
        return total_loss / (len(data_source) - 1)

    best_val_loss = float("inf")
    epochs = 100 # The number of epochs
    best_model = None

    for epoch in range(1, epochs + 1):
        print("EPOCH", epoch)
        epoch_start_time = time.time()
        train()
        scheduler.step()
