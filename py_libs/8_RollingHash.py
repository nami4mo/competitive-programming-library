class RollingHash():
    def __init__(self, s, base1=1007, base2=2009, mod1=10**9+7, mod2=10**9+9):
        self.mod1 = mod1
        self.mod2 = mod2
        self.s = s
        n = len(s)
        self.hash1 = [0]*(n+1)
        self.pow1 = [1]*(n+1)
        self.hash2 = [0]*(n+1)
        self.pow2 = [1]*(n+1)
        for i in range(n):
            self.hash1[i+1] = (self.hash1[i]*base1 + ord(s[i])) % self.mod1
            self.pow1[i+1] = (self.pow1[i]*base1) % self.mod1
            self.hash2[i+1] = (self.hash2[i]*base2 + ord(s[i])) % self.mod2
            self.pow2[i+1] = (self.pow2[i]*base2) % self.mod2

    def get(self, l, r):
        h1 = (self.hash1[r] - self.hash1[l] * self.pow1[r-l]) % self.mod1
        h2 = (self.hash2[r] - self.hash2[l] * self.pow2[r-l]) % self.mod2
        return (h1,h2)

# s = 'abcdefghijk'
# t = 'cde'
# rhs = RollingHash(s)
# rht = RollingHash(t)

# th = rht.get(0,len(t))
# for i in range(len(s)-len(t)+1):
#     sh = rhs.get(i,i+len(t))
#     if th == sh:
#         print(i)