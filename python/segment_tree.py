class SegmentTree():
    def __init__(self, data, padding=0):
        self.n = len(data)
        self.N = 2 ** (self.n-1).bit_length()
        self.padding = padding
        self.data = [self.padding] * (self.N - 1) + data + [self.padding] * (self.N - self.n)
        for i in range(2*self.N-2, 0, -2):
            self.data[(i-1)//2] = self.data[i] + self.data[i-1]

    def __len__(self):
        return self.n

    def __getitem__(self, i):
        return self.data[self.N - 1 + i]

    def __setitem__(self, i, val):
        idx = self.N - 1 + i
        self.data[idx] = val
        while idx > 0:
            idx = (idx - 1) // 2
            self.data[idx] = self.data[2*idx+1] + self.data[2*idx+2]

    def query(self, i, j):
        if i == j:
            return self.data[self.N + i - 1]
        else:
            idxi = self.N + i - 1
            idxj = self.N + j - 2
            result = self.padding
            while idxi < idxj + 1:
                if idxi % 2 == 0:
                    result += self.data[idxi]
                if idxj % 2 == 1:
                    result += self.data[idxj]
                    idxj -= 1
                idxi //= 2
                idxj = (idxj - 1) // 2
            return result
