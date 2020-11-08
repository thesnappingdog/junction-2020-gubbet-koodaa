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
import numpy as np

if __name__ == "__main__":
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")

    batch_size = 128
    bptt = 80

    ntoken_embedding = None #len(TEXT.vocab.stoi) # the size of vocabulary # AMOUNT OF COMMANDS
    embedding_size = 24 # embedding dimension
    nhid = 256 # the dimension of the feedforward network model in nn.TransformerEncoder
    nlayers = 6 # the number of nn.TransformerEncoderLayer in nn.TransformerEncoder
    nhead = 8 # the number of heads in the multiheadattention models
    dropout = 0.2 # the dropout value
    model = TransformerModel(ntoken_embedding, embedding_size, nhead, nhid, nlayers, n_outputs=30, ready_embedding=True, dropout=0.2, bptt=bptt).to(device)


    criterion = nn.CrossEntropyLoss()
    #lr = 5.0 # learning rate
    #optimizer = torch.optim.SGD(model.parameters(), lr=lr)
    optimizer = torch.optim.Adam(model.parameters(), lr=0.001, betas=(0.9, 0.98), eps=1e-09)
    #scheduler = torch.optim.lr_scheduler.StepLR(optimizer, 1.0, gamma=0.95)

    dataloader = DataLoader(
        FilterBankDataset(), 
        batch_size=batch_size,
        shuffle=True,
        collate_fn=collate)



    def train():
        model.train() # Turn on the train mode
        total_loss = 0.
        start_time = time.time()
        src_mask = model.generate_square_subsequent_mask(bptt).to(device)
        accuracies = []
        for i, batch in enumerate(dataloader):

            data = batch[0].to(device)
            targets = batch[1].to(device)
            optimizer.zero_grad()
            if data.size(0) != bptt:
                src_mask = model.generate_square_subsequent_mask(data.size(0)).to(device)
                print("funky mask", src_mask.size())
            output = model(data, src_mask)
            #print("Pred", torch.argmax(output, dim=1))
            #print("Targ", targets)
            loss = criterion(output, targets)
            corr = torch.argmax(output, dim=1) == targets
            corr_len = torch.where(corr == True)[0].size()[0]
            #print("corr", corr_len, corr_len/batch_size)
            accuracies.append(corr_len/data.size()[1])
            #print("Loss", loss.item(), "\n")
            loss.backward()
            torch.nn.utils.clip_grad_norm_(model.parameters(), 0.5)
            optimizer.step()

            total_loss += loss.item()
        mean_acc = np.mean(accuracies)
        return model, mean_acc

    best_accuracy= -float("inf")
    epochs = 1000 # The number of epochs
    best_model = None

    for epoch in range(1, epochs + 1):
        print("EPOCH", epoch)
        epoch_start_time = time.time()
        model, mean_acc = train()
        print("Mean accuracy", mean_acc)
        if mean_acc > best_accuracy:
            torch.save(model.state_dict(), "best_model.mdl")
            print("Saved model")
            best_accuracy = mean_acc

        #scheduler.step()
