import math
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.nn import TransformerEncoder, TransformerEncoderLayer
from positional_encoding import PositionalEncoding

class TransformerModel(nn.Module):

    def __init__(self, ntoken_embedding, embedding_size, nhead, nhid, nlayers, n_outputs=2, ready_embedding=False, dropout=0.5, batch_size=1, bptt=684):
        super(TransformerModel, self).__init__()
        self.model_type = 'Transformer'
        self.pos_encoder = PositionalEncoding(embedding_size, dropout)
        encoder_layers = TransformerEncoderLayer(embedding_size, nhead, nhid, dropout)
        self.transformer_encoder = TransformerEncoder(encoder_layers, nlayers)
        self.ready_embedding = ready_embedding
        if not self.ready_embedding:
            self.encoder = nn.Embedding(ntoken_embedding, embedding_size)
            self.ninp = ninp
        self.decoder = nn.Linear(embedding_size*bptt, n_outputs)
        self.batch_size = batch_size

        self.init_weights()

    def generate_square_subsequent_mask(self, sz):
        mask = (torch.triu(torch.ones(sz, sz)) == 1).transpose(0, 1)
        mask = mask.float().masked_fill(mask == 0, float('-inf')).masked_fill(mask == 1, float(0.0))
        return mask

    def init_weights(self):
        initrange = 0.1
        if not self.ready_embedding:
            self.encoder.weight.data.uniform_(-initrange, initrange)
        self.decoder.bias.data.zero_()
        self.decoder.weight.data.uniform_(-initrange, initrange)

    def forward(self, src, src_mask):
        if not self.ready_embedding:
            src = self.encoder(src) * math.sqrt(self.ninp)
        src = self.pos_encoder(src)
        output = self.transformer_encoder(src, src_mask)
        output = output.view((self.batch_size,-1))
        #print("outp size", output.size(), self.decoder)
        output = self.decoder(output)
        return output